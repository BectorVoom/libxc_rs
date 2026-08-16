//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 726/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk726(t1268: f64, t6336: f64, t6352: f64, t3516: f64, t6418: f64, t6422: f64, t538: f64, t6492: f64, t6442: f64, t2061: f64, t25: f64, t3472: f64, t3508: f64, t3543: f64, t4600: f64, t4607: f64, t5072: f64, t5076: f64, t6502: f64, t6505: f64, t6508: f64, t6510: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6513 = t1268 * t6336;
    let t6516 = t1268 * t6352;
    let t6519 = t3516 * t6418;
    let t6522 = t1268 * t6422;
    let t6525 = t538 * t6492;
    let t6528 = t538 * t6442;
    let t6531 = 0.017777777777777778_f64 * t5072 - 0.03199259259259259_f64 * t4600 + 0.047988888888888886_f64 * t4607 - 0.014814814814814815_f64 * t5076 - t3472 - t3543 - 0.007407407407407408_f64 * t3508 + 0.14396666666666666_f64 * t6502 - 0.03999074074074074_f64 * t6505 - 0.09597777777777777_f64 * t6508 + 0.013333333333333334_f64 * t25 * t6510 - 0.0022222222222222222_f64 * t25 * t6513 + 0.013333333333333334_f64 * t25 * t6516 - 0.002962962962962963_f64 * t25 * t6519 - 0.008888888888888889_f64 * t2061 * t6522 - 0.04_f64 * t25 * t6525 + 0.05333333333333334_f64 * t2061 * t6528;
    (t6513, t6516, t6519, t6522, t6525, t6528, t6531)
}
