//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 997/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk997(t11636: f64, t41: f64, t479: f64, t5451: f64, t1590: f64, t1905: f64, t164: f64, t4437: f64, t10749: f64, t10750: f64, t10755: f64, t10757: f64, t10760: f64, t11621: f64, t11623: f64, t11626: f64, t11627: f64, t11629: f64, t11631: f64, t11633: f64) -> (f64, f64) {
    let t11637 = t41 * t11636;
    let t11640 = t5451 * t479;
    let t11642 = t1905 * t1590;
    let t11643 = 0.09451622166942335_f64 * t11642;
    let t11644 = t4437 * t164;
    let t11648 = -t11621 + 0.09451622166942335_f64 * t11623 + t11626 + 0.09451622166942335_f64 * t11627 - 0.1890324433388467_f64 * t11629 - 0.00011865309871651405_f64 * t11631 - t10749 - 0.031505407223141116_f64 * t11633 - 0.031505407223141116_f64 * t11637 * t164 - 0.09451622166942335_f64 * t11640 - t11643 + 0.1890324433388467_f64 * t11644 - 0.09451622166942335_f64 * t10750 - t10755 + 0.5670973300165402_f64 * t10757 + t10760;
    (t11637, t11648)
}
