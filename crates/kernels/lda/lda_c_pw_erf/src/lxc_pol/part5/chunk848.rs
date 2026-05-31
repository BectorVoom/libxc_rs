//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 848/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk848<F: Float>(t2407: F, t808: F, t2120: F, t2505: F, t6209: F, t7797: F, t220: F, t186: F, t548: F, t6895: F, t6897: F, t5340: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7831 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t2407 * t808;
    let t7833 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t2120 * t2505;
    let t7835 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t6209 * t2505;
    let t7836 = -t7797;
    let t7837 = t220 * t7836;
    let t7838 = t186 * t7837;
    let t7840 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t548 * t7838;
    let t7841 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t6895;
    let t7842 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t6897;
    let t7843 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t5340;
    (t7831, t7833, t7835, t7836, t7837, t7838, t7840, t7841, t7842, t7843)
}
