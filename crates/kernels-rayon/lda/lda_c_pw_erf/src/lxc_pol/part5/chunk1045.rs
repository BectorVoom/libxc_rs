//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1045/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1045(t164: f64, t6138: f64, t684: f64, t7071: f64, t1553: f64, t1878: f64, t2610: f64, t2765: f64, t440: f64, t2644: f64, t405: f64, t7213: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18788 = t6138 * t164;
    let t18795 = t684 * t7071;
    let t18797 = t1553 * t1878;
    let t18805 = t2765 * t2610 * t440;
    let t18809 = t405 * t2644 * t1553;
    let t18830 = t405 * t7213;
    (t18788, t18795, t18797, t18805, t18809, t18830)
}
