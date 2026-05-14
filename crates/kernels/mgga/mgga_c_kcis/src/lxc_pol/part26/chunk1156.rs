//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1156/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1156<F: Float>(t1464: F, t1497: F, t60761: F, t7923: F, t16622: F, t28504: F, t491: F, t28338: F, t98470: F, t1928: F, t4122: F, t98409: F, t1394: F, t8164: F, t98020: F, t28331: F, t28356: F, t5780: F) -> (F, F, F, F, F, F, F) {
    let t102102 = t1464 * t7923 * t60761 * t1497;
    let t102106 = t1464 * t16622 * t491 * t28504;
    let t102109 = t1464 * t98470 * t28338;
    let t102115 = t1464 * t4122 * t1928 * t28504;
    let t102118 = t1464 * t98409 * t28338;
    let t102121 = t1394 * t98020 * t8164;
    let t102124 = t5780 * t28356 * t28331;
    (t102102, t102106, t102109, t102115, t102118, t102121, t102124)
}
