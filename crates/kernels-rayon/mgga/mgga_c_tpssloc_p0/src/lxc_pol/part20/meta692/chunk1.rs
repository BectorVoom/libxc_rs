//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2636/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2636(t11605: f64, t1760: f64, t11868: f64, t1190: f64, t11918: f64, t11919: f64, t11928: f64, t11934: f64, t1238: f64, t14972: f64, t15771: f64, t15787: f64, t15790: f64, t1720: f64, t1761: f64, t27784: f64, t3487: f64, t3590: f64, t3593: f64, t3598: f64, t3631: f64, t45345: f64, t45355: f64, t45375: f64, t4940: f64, t498: f64, t5055: f64, t5089: f64) -> f64 {
    let t53677 = t11605 * t1760;
    let t53697 = 2.0_f64 * t11918 * t1238 * t1760 * t3598 + t11868 * t1720 * t498 + 3.0_f64 * t1190 * t15771 * t498 - 18.0_f64 * t11934 * t27784 * t53677 + 3.0_f64 * t3590 * t4940 * t498 - t11919 * t5055 - 3.0_f64 * t11928 * t5089 - 3.0_f64 * t14972 * t3631 - 3.0_f64 * t15787 * t3593 + 12.0_f64 * t15790 * t3487 - 3.0_f64 * t1761 * t45345 - 3.0_f64 * t1761 * t45355 - t1761 * t45375;
    t53697
}
