//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 937/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk937(t1309: f64, t1324: f64, t13795: f64, t13851: f64, t13856: f64, t13859: f64, t13861: f64, t13863: f64, t13866: f64, t13868: f64, t13873: f64, t13880: f64, t13886: f64, t3944: f64, t3948: f64, t3966: f64, t405: f64) -> f64 {
    let t13889 = 0.53972366148531951639e-1_f64 * t13851 - 0.24627390775922727564e1_f64 * t13856 * t405 + 0.52772980234120130492e0_f64 * t13859 - 0.35981577432354634426e-1_f64 * t13861 + 0.15831894070236039148e1_f64 * t13863 * t405 - 0.28785261945883707541e0_f64 * t13866 + 0.95950873152945691803e-1_f64 * t13868 + t13873 - 0.16191709844559585492e0_f64 * t13795 * t1324 - 0.10794473229706390328e0_f64 * t3966 * t3944 - 0.1439263097294185377e0_f64 * t1309 * t13880 + 0.53972366148531951639e-1_f64 * t3966 * t3948 + 0.17990788716177317213e-1_f64 * t1309 * t13886;
    t13889
}
