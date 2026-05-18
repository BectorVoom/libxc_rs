//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1202/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1202<F: Float>(t1780: F, t932: F, t143: F, t19107: F, t346: F, t10631: F, t1143: F, t14907: F, t14966: F, t14996: F, t15046: F, t18542: F, t18555: F, t18596: F, t18600: F, t18753: F, t18788: F, t3381: F, t348: F, t4602: F, t4607: F, t4638: F, t4643: F, t4671: F, t5111: F, t5122: F, t6589: F) -> F {
    let t20098 = t1780 * t932;
    let t20107 = t19107 * t143;
    let t20112 = t1780 * t346;
    let t20126 = F::new(0.11791604938271604938e-1) * t14966 - F::new(0.9286875e-2) * t3381 * t18596 + F::new(0.17687407407407407407e-1) * t14996 + F::new(0.9286875e-2) * t20098 * t4602 - F::new(0.1857375e-1) * t3381 * t18542 + F::new(0.46434375e-2) * t5111 * t18788 + F::new(0.123825e-1) * t5122 * t18600 + F::new(0.619125e-2) * t20107 * t348 + F::new(0.24765e-1) * t5122 * t18555 + F::new(0.24765e-1) * t20112 * t4643 - F::new(0.1857375e-1) * t14907 * t4638 - F::new(0.1857375e-1) * t3381 * t18753 + F::new(0.88437037037037037037e-2) * t10631 - t15046 - F::new(0.123825e-1) * t1780 * t4671 + F::new(0.46434375e-2) * t1143 * t6589 - F::new(0.1857375e-1) * t14907 * t4607;
    t20126
}
