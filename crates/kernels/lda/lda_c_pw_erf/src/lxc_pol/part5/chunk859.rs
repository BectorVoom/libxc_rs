//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 859/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk859<F: Float>(t770: F, t142: F, t2776: F, t2630: F, t1554: F, t100: F, t7918: F, t2675: F, t6086: F, t1809: F, t2591: F, t5799: F, t5801: F, t6162: F, t7460: F, t7462: F, t7464: F, t7468: F, t7472: F, t7473: F, t7477: F, t7481: F, t7483: F, t7487: F, t7491: F, t7493: F, t7494: F, t7495: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7986 = t770 * t770;
    let t7987 = t142 * t7986;
    let t7988 = t2776 * t7987;
    let t7991 = t142 * t2630;
    let t7992 = t1554 * t7991;
    let t7996 = t7918 * t100;
    let t8001 = t6086 * t2675;
    let t8004 = t2591 * t1809;
    let t8010 = t5799 + F::cast_from(0.36466666666666664_f64) * t5801 - t7460 - t7462 - t7464 + t7468 + t7472 - t7473 + t7477 + t7481 - t7483 - t7487 - t7491 - t7493 - F::new(2.0) / F::new(15.0) * t6162 + t7494 + t7495;
    (t7986, t7987, t7988, t7991, t7992, t7996, t8001, t8004, t8010)
}
