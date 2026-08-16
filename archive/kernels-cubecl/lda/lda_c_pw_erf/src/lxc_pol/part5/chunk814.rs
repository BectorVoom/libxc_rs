//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 814/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk814<F: Float>(t7433: F, t7455: F, t582: F, t186: F, t211: F, t2443: F, t808: F, t2528: F, t822: F, t2405: F, t793: F, t184: F) -> (F, F, F, F, F, F, F, F) {
    let t7456 = t7433 + t7455;
    let t7457 = t582 * t7456;
    let t7458 = t186 * t7457;
    let t7460 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t211 * t7458;
    let t7462 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t2443 * t808;
    let t7464 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t822 * t2528;
    let t7465 = t2405 * t793;
    let t7466 = t7465 * t184;
    (t7456, t7457, t7458, t7460, t7462, t7464, t7465, t7466)
}
