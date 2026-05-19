//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 726/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk726<F: Float>(t1268: F, t6336: F, t6352: F, t3516: F, t6418: F, t6422: F, t538: F, t6492: F, t6442: F, t2061: F, t25: F, t3472: F, t3508: F, t3543: F, t4600: F, t4607: F, t5072: F, t5076: F, t6502: F, t6505: F, t6508: F, t6510: F) -> (F, F, F, F, F, F, F) {
    let t6513 = t1268 * t6336;
    let t6516 = t1268 * t6352;
    let t6519 = t3516 * t6418;
    let t6522 = t1268 * t6422;
    let t6525 = t538 * t6492;
    let t6528 = t538 * t6442;
    let t6531 = F::cast_from(0.017777777777777778_f64) * t5072 - F::cast_from(0.03199259259259259_f64) * t4600 + F::cast_from(0.047988888888888886_f64) * t4607 - F::cast_from(0.014814814814814815_f64) * t5076 - t3472 - t3543 - F::cast_from(0.007407407407407408_f64) * t3508 + F::cast_from(0.14396666666666666_f64) * t6502 - F::cast_from(0.03999074074074074_f64) * t6505 - F::cast_from(0.09597777777777777_f64) * t6508 + F::cast_from(0.013333333333333334_f64) * t25 * t6510 - F::cast_from(0.0022222222222222222_f64) * t25 * t6513 + F::cast_from(0.013333333333333334_f64) * t25 * t6516 - F::cast_from(0.002962962962962963_f64) * t25 * t6519 - F::cast_from(0.008888888888888889_f64) * t2061 * t6522 - F::new(0.04) * t25 * t6525 + F::cast_from(0.05333333333333334_f64) * t2061 * t6528;
    (t6513, t6516, t6519, t6522, t6525, t6528, t6531)
}
