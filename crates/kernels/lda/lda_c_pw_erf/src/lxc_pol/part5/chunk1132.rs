//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1132/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1132<F: Float>(t21393: F, t21396: F, t21401: F, t21403: F, t21406: F, t21409: F, t21413: F, t21417: F, t21423: F, t21426: F, t21427: F, t21428: F, t21430: F, t12810: F, t12815: F, t12839: F, t12863: F, t12870: F, t21431: F, t21432: F, t21436: F, t21438: F, t21442: F, t21445: F, t21447: F, t21449: F) -> (F, F) {
    let t23246 = t21393 - t21396 + t21401 - t21403 + t21406 - t21409 + t21413 + t21417 + t21423 + t21426 - t21427 + t21428 - t21430;
    let t23247 = -t21431 - t21432 + t21436 - t21438 + t21442 + t21445 + t21447 - t21449 - t12810 + t12815 + t12839 + t12863 - t12870;
    (t23246, t23247)
}
