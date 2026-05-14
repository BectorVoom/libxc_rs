//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1049/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1049<F: Float>(t11: F, t41: F, t85: F, t5143: F, t5135: F, t10552: F, t10554: F, t10556: F, t10558: F, t10599: F, t1153: F, t14118: F, t14242: F, t14287: F, t14922: F, t14926: F, t14927: F, t14930: F, t14940: F, t14944: F, t14949: F, t2429: F, t3381: F, t5122: F) -> (F, F) {
    let t14954 = t11 * t41;
    let t14955 = t85 * t14954;
    let t14956 = t14955 * t5143;
    let t14959 = 0.5895802469135802469e-1 * t14955 * t5135;
    let t14960 = -0.10612444444444444444e0 * t2429 * t14922 - t14926 + 0.88437037037037037036e-1 * t14927 - 0.26531111111111111111e-1 * t1153 * t14930 - 0.17687407407407407407e-1 * t10552 - 0.29479012345679012345e-1 * t10554 + 0.11791604938271604938e-1 * t10556 - 0.35374814814814814814e-1 * t10558 + 0.35374814814814814814e-1 * t10599 - 0.9286875e-2 * t3381 * t14242 - 0.232171875e-2 * t14940 * t14118 - 0.26531111111111111111e-1 * t1153 * t14944 - 0.44218518518518518518e-1 * t1153 * t14949 + 0.123825e-1 * t5122 * t14287 + 0.70749629629629629629e-1 * t14956 - t14959;
    (t14955, t14960)
}
