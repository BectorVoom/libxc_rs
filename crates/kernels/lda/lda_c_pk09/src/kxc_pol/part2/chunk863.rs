//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 863/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk863<F: Float>(t10465: F, t306: F, t1277: F, t309: F, t310: F, t1382: F, t2487: F, t1486: F, t9602: F, t1287: F, t9922: F, t9929: F, t5047: F, t5071: F, t5348: F, t5361: F, t5362: F, t5367: F, t5370: F, t9628: F, t9746: F, t9753: F, t9756: F, t9925: F, t9933: F, t9936: F, t9943: F) -> (F, F, F, F, F) {
    let t10466 = t10465 * t306;
    let t10468 = t309 * t310 * t1277;
    let t10471 = t1382 * t2487;
    let t10474 = t1486 * t9602;
    let t10475 = t10474 * t1287;
    let t10479 = 8.0 * t9922;
    let t10481 = 8.0 * t9929;
    let t10489 = -t5362 + t5367 + t5348 + t5361 + 0.821419393556371 * t5047 - t5370 + 0.2738064645187903 * t5071 + t10479 - 8.0 * t9925 - t10481 + 12.0 * t9933 - 8.0 * t9936 + 0.821419393556371 * t9746 + 0.2738064645187903 * t9753 + 0.821419393556371 * t9756 + 1.642838787112742 * t9628 - 2.6666666666666665 * t9943;
    (t10466, t10468, t10471, t10475, t10489)
}
