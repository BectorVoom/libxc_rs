//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 990/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk990<F: Float>(t1330: F, t9814: F, t306: F, t1215: F, t2606: F, t5047: F, t5071: F, t6078: F, t6091: F, t6092: F, t6097: F, t6100: F, t9628: F, t9746: F, t9753: F, t9756: F, t9922: F, t9925: F, t9929: F, t9933: F, t9936: F, t9943: F) -> (F, F, F) {
    let t10622 = t9814 * t1330;
    let t10623 = t10622 * t306;
    let t10626 = t2606 * t1215;
    let t10641 = -t6092 + t6097 + t6078 + t6091 + F::cast_from(0.64_f64) * t5047 - t6100 + F::cast_from(0.21333333333333335_f64) * t5071 + F::cast_from(6.416968383055361_f64) * t9922 - F::cast_from(6.416968383055361_f64) * t9925 - F::cast_from(6.416968383055361_f64) * t9929 + F::cast_from(9.625452574583042_f64) * t9933 - F::cast_from(6.416968383055361_f64) * t9936 + F::cast_from(0.64_f64) * t9746 + F::cast_from(0.21333333333333335_f64) * t9753 + F::cast_from(0.64_f64) * t9756 + F::cast_from(1.28_f64) * t9628 - F::cast_from(2.1389894610184537_f64) * t9943;
    (t10623, t10626, t10641)
}
