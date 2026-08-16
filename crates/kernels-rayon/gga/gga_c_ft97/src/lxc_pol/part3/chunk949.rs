//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 949/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk949(t18659: f64, t242: f64, t3859: f64, t3977: f64, t729: f64, t1175: f64, t3746: f64, t724: f64, t13886: f64, t13885: f64, t1131: f64, t2567: f64) -> (f64, f64, f64, f64, f64) {
    let t18660 = t242 * t18659;
    let t18664 = t729 * t3977 * t3859;
    let t18668 = t724 * t1175 * t3746;
    let t18671 = t13886 * t3859;
    let t18672 = t13885 * t18671;
    let t18675 = t2567 * t1131;
    (t18660, t18664, t18668, t18672, t18675)
}
