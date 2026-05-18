//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1014/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1014<F: Float>(t1587: F, t398: F, t1591: F, t4376: F, t1586: F, t4374: F, t1163: F, t1312: F, t1390: F, t1588: F, t14910: F, t1572: F, t4420: F) -> (F, F, F, F) {
    let t14961 = t1587 * t1587;
    let t14962 = F::new(1.0) / t14961;
    let t14963 = t398 * t14962;
    let t14964 = t4376 * t1591;
    let t14965 = t14963 * t14964;
    let t14966 = t1586 * t14965;
    let t14971 = t4374 * t4376;
    let t14972 = t14971 * t1163;
    let t14973 = t1312 * t14972;
    let t14978 = t1588 * t1390;
    let t14979 = t14978 * t14910;
    let t14980 = t1312 * t14979;
    let t14983 = t1572 * t4420;
    (t14966, t14973, t14980, t14983)
}
