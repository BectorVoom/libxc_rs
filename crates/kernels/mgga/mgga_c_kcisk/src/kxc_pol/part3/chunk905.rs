//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 905/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk905<F: Float>(t14920: F, t14959: F, t15004: F, t15072: F, t1597: F, t14230: F, t14232: F, t14237: F, t14247: F, t14250: F, t14253: F, t14258: F, t14262: F, t14268: F, t14271: F, t14866: F, t1557: F, t548: F) -> (F,) {
    let t15074 = t14920 + t14959 + t15004 + t15072;
    let t15075 = t15074 * t1597;
    let t15079 = -0.46429444444444444443e-2 * t14230 - 0.12381185185185185185e-1 * t14232 + 0.69644166666666666665e-2 * t14237 + 0.34048259259259259259e-1 * t14247 + t14866 * t548 + 0.30952962962962962963e-2 * t14250 + 0.51072388888888888887e-1 * t14253 + 0.38691203703703703703e-2 * t14258 - 0.77382407407407407405e-3 * t14262 + 0.69644166666666666665e-2 * t14268 - 0.193e0 * t1557 * t15075 - 0.13928833333333333333e-1 * t14271;
    (t15079,)
}
