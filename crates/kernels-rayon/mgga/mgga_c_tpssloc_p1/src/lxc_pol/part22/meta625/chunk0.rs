//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2159/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2159(t53782: f64, t16169: f64, t2663: f64, t15892: f64, t2371: f64, t5154: f64, t9919: f64, t12344: f64, t5234: f64, t1369: f64, t1831: f64, t40059: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t53783 = 0.32530743900905219526e-1_f64 * t53782;
    let t53787 = t16169 * t2663;
    let t53788 = 0.73245789224026180216e-3_f64 * t53787;
    let t53796 = t15892 * t2371;
    let t53797 = 0.35089341735807877242e1_f64 * t53796;
    let t53798 = t5154 * t9919;
    let t53880 = t5234 * t12344;
    let t53881 = t53880 * t1369;
    let t53882 = 119.0_f64 / 1152.0_f64 * t53881;
    let t53901 = t40059 * t1831;
    (t53783, t53788, t53797, t53798, t53880, t53882, t53901)
}
