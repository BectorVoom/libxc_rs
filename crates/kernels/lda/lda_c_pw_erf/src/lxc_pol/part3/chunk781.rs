//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 781/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk781<F: Float>(t4389: F, t4391: F, t4398: F, t4401: F, t4403: F, t4406: F, t4408: F, t4412: F, t4416: F, t4418: F, t5690: F, t5695: F, t2696: F, t2699: F, t2702: F, t2708: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7325 = 1.7544670192365612 * t4389;
    let t7326 = 51.94726769812759 * t4391;
    let t7327 = 0.032530742648344574 * t4398;
    let t7328 = 36.0 * t4401;
    let t7329 = 96.0 * t4403;
    let t7330 = 3.0 * t4406;
    let t7332 = 60.0 * t4408;
    let t7333 = 3.5089340384731225 * t4412;
    let t7334 = 1.898172889849454 * t4416;
    let t7335 = 2.0538164420033334 * t4418;
    let t7350 = 24.0 * t5690;
    let t7353 = 24.0 * t5695;
    let t8097 = 1.8960024086108225 * t2696;
    let t8098 = 0.06506148529668915 * t2699;
    let t8099 = 1.9263778438055648 * t2702;
    let t8101 = 0.1301229705933783 * t2708;
    (t7325, t7326, t7327, t7328, t7329, t7330, t7332, t7333, t7334, t7335, t7350, t7353, t8097, t8098, t8099, t8101)
}
