//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 937/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk937<F: Float>(t1309: F, t1324: F, t13795: F, t13851: F, t13856: F, t13859: F, t13861: F, t13863: F, t13866: F, t13868: F, t13873: F, t13880: F, t13886: F, t3944: F, t3948: F, t3966: F, t405: F) -> F {
    let t13889 = F::cast_from(0.53972366148531951639e-1_f64) * t13851 - F::cast_from(0.24627390775922727564e1_f64) * t13856 * t405 + F::cast_from(0.52772980234120130492e0_f64) * t13859 - F::cast_from(0.35981577432354634426e-1_f64) * t13861 + F::cast_from(0.15831894070236039148e1_f64) * t13863 * t405 - F::cast_from(0.28785261945883707541e0_f64) * t13866 + F::cast_from(0.95950873152945691803e-1_f64) * t13868 + t13873 - F::cast_from(0.16191709844559585492e0_f64) * t13795 * t1324 - F::cast_from(0.10794473229706390328e0_f64) * t3966 * t3944 - F::cast_from(0.1439263097294185377e0_f64) * t1309 * t13880 + F::cast_from(0.53972366148531951639e-1_f64) * t3966 * t3948 + F::cast_from(0.17990788716177317213e-1_f64) * t1309 * t13886;
    t13889
}
