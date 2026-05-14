//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1016/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1016<F: Float>(t11717: F, t7333: F, t16648: F, t7316: F, t7315: F, t11763: F, t7299: F, t11744: F, t2580: F, t2587: F, t5307: F, t16568: F, t716: F, t740: F, t748: F, t1945: F, t7413: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17949 = t11717 * t7333;
    let t17951 = t7316 * t16648;
    let t17952 = t7315 * t17951;
    let t17954 = t11763 * t7299;
    let t17956 = t11744 * t2580;
    let t17958 = t5307 * t2587;
    let t17960 = t16568 * t716;
    let t17961 = t17960 * t740;
    let t17962 = t17961 * t748;
    let t17964 = t1945 * t7413;
    (t17949, t17951, t17952, t17954, t17956, t17958, t17960, t17962, t17964)
}
