//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1092/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1092(t2843: f64, t4299: f64, t7679: f64, t1466: f64, t36056: f64, t681: f64, t28729: f64, t33961: f64, t143040: f64, t143158: f64, t35819: f64, t684: f64) -> (f64, f64, f64, f64, f64) {
    let t152648 = t2843 * t7679 * t4299;
    let t152651 = t1466 * t681 * t36056;
    let t152657 = t33961 * t28729;
    let t152659 = t143040 * t143158 * t152657;
    let t152661 = t35819 * t684;
    (t152648, t152651, t152657, t152659, t152661)
}
