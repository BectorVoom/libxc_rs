//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 990/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk990(t1330: f64, t9814: f64, t306: f64, t1215: f64, t2606: f64, t5047: f64, t5071: f64, t6078: f64, t6091: f64, t6092: f64, t6097: f64, t6100: f64, t9628: f64, t9746: f64, t9753: f64, t9756: f64, t9922: f64, t9925: f64, t9929: f64, t9933: f64, t9936: f64, t9943: f64) -> (f64, f64, f64) {
    let t10622 = t9814 * t1330;
    let t10623 = t10622 * t306;
    let t10626 = t2606 * t1215;
    let t10641 = -t6092 + t6097 + t6078 + t6091 + 0.64_f64 * t5047 - t6100 + 0.21333333333333335_f64 * t5071 + 6.416968383055361_f64 * t9922 - 6.416968383055361_f64 * t9925 - 6.416968383055361_f64 * t9929 + 9.625452574583042_f64 * t9933 - 6.416968383055361_f64 * t9936 + 0.64_f64 * t9746 + 0.21333333333333335_f64 * t9753 + 0.64_f64 * t9756 + 1.28_f64 * t9628 - 2.1389894610184537_f64 * t9943;
    (t10623, t10626, t10641)
}
