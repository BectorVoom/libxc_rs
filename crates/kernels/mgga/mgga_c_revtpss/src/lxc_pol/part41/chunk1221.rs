//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1221/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1221<F: Float>(t12261: F, t12297: F, t16706: F, t16869: F, t16873: F, t16876: F, t20268: F, t20274: F, t20276: F, t20278: F, t20280: F, t20338: F, t20341: F, t20344: F, t20347: F, t20350: F, t20353: F, t20357: F, t20359: F, t20362: F, t20425: F, t20445: F) -> (F,) {
    let t20447 = 0.91285185185185185187e-1 * t12261 - t16869 - t16873 - 0.27385555555555555556e-1 * t20268 + 0.26574814814814814815e0 * t16706 + 0.18257037037037037037e0 * t16876 + 0.82156666666666666667e-1 * t20274 + 0.18257037037037037037e-1 * t20276 - 0.10954222222222222222e0 * t20278 - 0.54771111111111111111e-1 * t20280 + t20425 + 0.1898925e1 * t20338 + 0.16431333333333333333e0 * t20341 - 0.54771111111111111112e-1 * t20344 - 0.16431333333333333333e0 * t20347 + 0.32862666666666666666e0 * t20350 + 0.49293999999999999999e0 * t20353 + 0.13287407407407407408e0 * t12297 + 0.142419375e1 * t20357 - 0.1898925e1 * t20359 - 0.9494625e0 * t20362 + t20445;
    (t20447,)
}
