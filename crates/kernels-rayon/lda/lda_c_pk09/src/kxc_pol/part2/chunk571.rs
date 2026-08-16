//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 571/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk571(t192: f64, t3230: f64, t3233: f64, t159: f64, t733: f64, t142: f64, t3163: f64, t35: f64, t68: f64, t889: f64, t3161: f64, t62: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3984 = t192 * t3230;
    let t3986 = t192 * t3233;
    let t3990 = t159 * t733;
    let t3991 = t3990 * t142;
    let t3993 = 9.87466743489671_f64 * t3991 * t3163;
    let t3994 = t35 * t35;
    let t3995 = 1.0_f64 / t3994;
    let t3996 = t3995 * t68;
    let t3997 = t3996 * t889;
    let t3998 = t62 * t3161;
    (t3984, t3986, t3993, t3995, t3997, t3998)
}
