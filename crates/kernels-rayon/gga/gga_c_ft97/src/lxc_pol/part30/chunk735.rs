//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 735/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk735(t3758: f64, t6783: f64, t17839: f64, t218: f64, t3762: f64, t1100: f64, t695: f64, t52: f64, t709: f64, t7457: f64, t1613: f64, t213: f64, t7464: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33383 = t3758 * t6783;
    let t33384 = t17839 * t218;
    let t33385 = t33384 * t3762;
    let t33388 = t1100 * t695;
    let t33390 = t52 * t7457 * t709;
    let t33394 = t1613 * t213 * t7464;
    (t33383, t33384, t33385, t33388, t33390, t33394)
}
