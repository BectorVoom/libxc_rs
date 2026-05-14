//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 749/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk749<F: Float>(t582: F, t7456: F, t186: F, t211: F, t2443: F, t808: F, t2528: F, t822: F, t2405: F, t793: F, t184: F, t199: F, t2400: F, t820: F, t221: F, t4465: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7457 = t582 * t7456;
    let t7458 = t186 * t7457;
    let t7460 = 2.0 / 15.0 * t211 * t7458;
    let t7462 = 2.0 / 5.0 * t2443 * t808;
    let t7464 = 2.0 / 5.0 * t822 * t2528;
    let t7465 = t2405 * t793;
    let t7466 = t7465 * t184;
    let t7468 = 4.0 / 5.0 * t7466 * t199;
    let t7469 = t2400 * t820;
    let t7470 = t7469 * t184;
    let t7472 = 4.0 / 5.0 * t7470 * t221;
    let t7473 = 4.0 / 45.0 * t4465;
    (t7457, t7458, t7460, t7462, t7464, t7465, t7466, t7468, t7469, t7470, t7472, t7473)
}
