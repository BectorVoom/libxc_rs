//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 773/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk773<F: Float>(t2505: F, t6209: F, t7797: F, t220: F, t186: F, t548: F, t6895: F, t6897: F, t5340: F, t5343: F, t7017: F, t7019: F, t231: F, t4235: F, t7278: F, t7818: F, t7820: F, t7824: F, t7826: F, t7827: F, t7831: F, t7833: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7835 = 4.0 / 5.0 * t6209 * t2505;
    let t7836 = -t7797;
    let t7837 = t220 * t7836;
    let t7838 = t186 * t7837;
    let t7840 = 4.0 / 15.0 * t548 * t7838;
    let t7841 = 16.0 / 15.0 * t6895;
    let t7842 = 16.0 / 45.0 * t6897;
    let t7843 = 8.0 / 135.0 * t5340;
    let t7844 = 8.0 / 135.0 * t5343;
    let t7846 = 4.0 / 15.0 * t7017;
    let t7847 = 16.0 / 15.0 * t7019;
    let t7848 = -t7818 + t7820 + t7824 - t7826 + t4235 + 4.0 / 3.0 * t7827 * t231 - t7831 + t7833 + t7835 + t7840 - t7841 + t7842 - t7843 - t7844 + 4.0 * t7278 - t7846 + t7847;
    (t7835, t7836, t7837, t7838, t7840, t7841, t7842, t7843, t7844, t7846, t7847, t7848)
}
