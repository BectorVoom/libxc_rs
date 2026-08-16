//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 712/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk712<F: Float>(t3578: F, t4953: F, t1222: F, t1731: F, t1744: F, t1202: F, t1743: F, t225: F, t4940: F, t68: F, t484: F, t1177: F, t4729: F) -> (F, F, F, F, F, F, F, F) {
    let t4954 = t3578 * t4953;
    let t4957 = t1731 * t1222;
    let t4959 = t1744 * t1222;
    let t4961 = t1202 * t1743;
    let t4964 = t4940 * t225;
    let t4965 = t4964 * t68;
    let t4966 = t4965 * t484;
    let t4969 = t1177 * t4729;
    (t4954, t4957, t4959, t4961, t4964, t4965, t4966, t4969)
}
