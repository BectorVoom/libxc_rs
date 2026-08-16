//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 381/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk381(t5778: f64, t5779: f64, t28: f64, t139: f64, t6: f64, t1995: f64, t1701: f64, t538: f64, t5546: f64, t5551: f64, t5555: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5780 = t5778 * t5779;
    let t5781 = t28 * t5780;
    let t5784 = t139 * t6;
    let t5785 = t1995 * t5784;
    let t5787 = t1701 * t5546 * t538;
    let t5790 = t5551 * t5555;
    (t5780, t5781, t5784, t5785, t5787, t5790)
}
