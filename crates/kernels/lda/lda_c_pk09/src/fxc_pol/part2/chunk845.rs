//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 845/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk845<F: Float>(t10082: F, t10098: F, t314: F, t306: F, t305: F, t9739: F, t304: F, t1215: F, t2567: F, t334: F, t9602: F, t1336: F, t1625: F, t2566: F, t1303: F, t5814: F, t9986: F) -> (F, F, F, F, F, F, F, F) {
    let t10099 = t10082 + t10098;
    let t10100 = t314 * t10099;
    let t10101 = t10100 * t306;
    let t10104 = t305 * t9739;
    let t10105 = t304 * t10104;
    let t10108 = t2567 * t1215;
    let t10116 = t9602 * t334;
    let t10119 = t2567 * t1336;
    let t10120 = t10119 * t1625;
    let t10124 = t2566 * t305;
    let t10125 = t1303 * t10124;
    let t10128 = t9986 * t5814;
    (t10101, t10104, t10105, t10108, t10116, t10120, t10125, t10128)
}
