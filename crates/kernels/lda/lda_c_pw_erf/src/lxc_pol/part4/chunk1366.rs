//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1366/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1366<F: Float>(t1549: F, t6093: F, t10988: F, t10991: F, t10992: F, t10995: F, t11486: F, t11511: F, t11588: F, t14465: F, t14488: F, t14505: F, t14508: F, t1881: F, t19832: F, t19847: F, t19850: F, t19860: F, t19864: F, t19866: F, t2211: F, t2765: F, t2766: F, t2777: F, t2809: F, t5735: F, t5740: F, t5783: F, t6025: F, t6087: F, t770: F) -> (F,) {
    let t19872 = t1549 * t6093;
    let t19875 = -0.31931290694012293 * t10988 - t10991 - 0.05321881782335382 * t10992 - t10995 - 6.0 * t19832 * t2765 * t770 * t2777 + 12.0 * t5735 * t5740 + 12.0 * t2211 * t11486 + 6.0 * t2211 * t11588 - 24.0 * t14488 * t14465 - 0.0005811348303577384 * t19847 - 0.0005811348303577384 * t19850 - 2.0 * t1881 * t6087 + 24.0 * t6025 * t14508 + 12.0 * t6025 * t14505 + 0.19816831758676853 * t19860 + 0.001355981270834723 * t19864 - 6.0 * t5783 * t19866 * t2766 + 6.0 * t2211 * t11511 + 6.0 * t2809 * t19872;
    (t19875,)
}
