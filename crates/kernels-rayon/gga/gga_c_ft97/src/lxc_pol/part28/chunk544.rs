//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 544/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk544(t14: f64, t22766: f64, t22632: f64, t5577: f64, t5580: f64, t397: f64, t5539: f64, t1613: f64, t5585: f64, t5584: f64, t1608: f64) -> (f64, f64, f64, f64, f64) {
    let t22767 = t22766 * t14;
    let t22775 = t5577 * t22632 * t5580;
    let t22777 = t5539 * t397;
    let t22794 = t5585 * t1613;
    let t22795 = t5584 * t22794;
    let t22796 = t1608 * t22795;
    (t22767, t22775, t22777, t22794, t22796)
}
