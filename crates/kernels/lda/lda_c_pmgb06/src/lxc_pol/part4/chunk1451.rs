//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1451/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1451<F: Float>(t18588: F, t5783: F, t11237: F, t1234: F, t1282: F, t14816: F, t18503: F, t18507: F, t18518: F, t18568: F, t18571: F, t18580: F, t18582: F, t18586: F, t2448: F, t2695: F, t3615: F, t370: F, t63: F, t8245: F) -> (F, F) {
    let t18589 = t5783 * t18588;
    let t18590 = F::new(3.8973666666666666) * t18589;
    let t18591 = t18503 - F::new(1.95872) * t11237 - t18507 + F::new(176.2848) * t63 * t8245 * t2695 * t1234 - F::new(29.3808) * t63 * t3615 * t2448 * t1234 + t18518 - t18571 - F::new(1.46904) * t63 * t370 * t18568 + F::new(11.75232) * t63 * t1282 * t14816 + t18580 - F::new(5.87616) * t18582 + t18586 - t18590;
    (t18590, t18591)
}
