//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 841/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk841<F: Float>(t13871: F, t396: F, t12951: F, t403: F, t12830: F, t3952: F, t12924: F, t1313: F, t1312: F, t1309: F, t1324: F, t13795: F, t13851: F, t13856: F, t13859: F, t13861: F, t13863: F, t13866: F, t13868: F, t3944: F, t3948: F, t3966: F, t405: F) -> (F,) {
    let t13873 = 0.19989765240197019125e-1 * t396 * t13871;
    let t13878 = t403 * t12951;
    let t13879 = t13878 * t12830;
    let t13880 = t3952 * t13879;
    let t13885 = t1313 * t12924;
    let t13886 = t1312 * t13885;
    let t13889 = 0.53972366148531951639e-1 * t13851 - 0.24627390775922727564e1 * t13856 * t405 + 0.52772980234120130492e0 * t13859 - 0.35981577432354634426e-1 * t13861 + 0.15831894070236039148e1 * t13863 * t405 - 0.28785261945883707541e0 * t13866 + 0.95950873152945691803e-1 * t13868 + t13873 - 0.16191709844559585492e0 * t13795 * t1324 - 0.10794473229706390328e0 * t3966 * t3944 - 0.1439263097294185377e0 * t1309 * t13880 + 0.53972366148531951639e-1 * t3966 * t3948 + 0.17990788716177317213e-1 * t1309 * t13886;
    (t13889,)
}
