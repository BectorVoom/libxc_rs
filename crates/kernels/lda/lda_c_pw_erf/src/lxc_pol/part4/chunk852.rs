//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 852/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk852<F: Float>(t2061: F, t25: F, t3472: F, t3508: F, t3543: F, t4600: F, t4607: F, t5072: F, t5076: F, t6502: F, t6505: F, t6508: F, t6510: F, t6513: F, t6516: F, t6519: F, t6522: F, t6525: F, t6528: F) -> (F,) {
    let t6531 = 0.017777777777777778 * t5072 - 0.03199259259259259 * t4600 + 0.047988888888888886 * t4607 - 0.014814814814814815 * t5076 - t3472 - t3543 - 0.007407407407407408 * t3508 + 0.14396666666666666 * t6502 - 0.03999074074074074 * t6505 - 0.09597777777777777 * t6508 + 0.013333333333333334 * t25 * t6510 - 0.0022222222222222222 * t25 * t6513 + 0.013333333333333334 * t25 * t6516 - 0.002962962962962963 * t25 * t6519 - 0.008888888888888889 * t2061 * t6522 - 0.04 * t25 * t6525 + 0.05333333333333334 * t2061 * t6528;
    (t6531,)
}
