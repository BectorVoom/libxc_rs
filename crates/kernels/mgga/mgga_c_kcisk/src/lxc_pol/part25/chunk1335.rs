//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1335/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1335<F: Float>(t34313: F, t5291: F, t4817: F, t7327: F, t654: F, t7400: F, t9709: F, t33094: F, t7304: F, t17843: F, t34368: F, t1944: F, t33120: F, t7312: F, t17065: F, t1950: F) -> (F, F, F, F, F, F, F) {
    let t117323 = t34313 * t5291;
    let t117325 = t4817 * t7327;
    let t117327 = t7400 * t654;
    let t117328 = t117327 * t9709;
    let t117330 = t33094 * t7304;
    let t117332 = t34368 * t17843;
    let t117335 = t1944 * t33120 * t7312;
    let t117337 = t17065 * t1950;
    (t117323, t117325, t117328, t117330, t117332, t117335, t117337)
}
