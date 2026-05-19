//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1223/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1223<F: Float>(t19740: F, t19741: F, t19742: F, t19746: F, t19748: F, t9457: F, t9461: F, t9467: F, t9470: F, t9478: F, t9481: F, t12225: F, t12227: F, t19935: F, t19937: F, t19939: F, t19941: F, t19943: F, t19944: F, t19945: F, t19946: F, t19947: F, t9483: F) -> (F, F) {
    let t21935 = -t19740 + t19741 + t19742 + t19746 + t19748 + F::cast_from(0.001515438175925926_f64) * t9457 + t9461 + t9467 + t9470 / F::new(3.0) + t9478 + t9481;
    let t21938 = F::cast_from(0.18233333333333332_f64) * t9483 - t12225 - F::new(2.0) / F::new(3.0) * t12227 - t19935 + t19937 - t19939 + t19941 + t19943 + t19944 + t19945 + t19946 + t19947;
    (t21935, t21938)
}
