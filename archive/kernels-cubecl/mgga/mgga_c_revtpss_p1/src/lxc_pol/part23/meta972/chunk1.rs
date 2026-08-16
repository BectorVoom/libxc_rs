//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3292/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3292<F: Float>(t1882: F, t6888: F, t22857: F, t555: F, t1399: F, t46505: F, t5675: F, t5745: F, t5755: F, t75021: F, t75024: F, t75026: F, t75035: F, t75039: F, t75041: F, t75049: F, t75053: F) -> (F, F, F) {
    let t86441 = t6888 * t1882;
    let t86445 = t555 * t22857;
    let t86453 = F::cast_from(0.39029762157531132076e-1_f64) * t75021 - F::cast_from(0.58544643236296698113e-1_f64) * t75024 + F::cast_from(0.19514881078765566037e-2_f64) * t75026 - F::cast_from(0.17563392970889009434e0_f64) * t75035 + F::cast_from(0.17563392970889009434e0_f64) * t75039 + F::cast_from(0.39512695097613069592e1_f64) * t5745 * t86441 * t5675 - F::cast_from(0.65854491829355115987e0_f64) * t5755 * t86445 * t1399 - F::cast_from(0.29272321618148349057e-1_f64) * t75041 + F::cast_from(0.46263278077393568556e-2_f64) * t46505 + F::cast_from(0.98781737744032673976e-1_f64) * t75049 - F::cast_from(0.98781737744032673976e-1_f64) * t75053;
    (t86441, t86445, t86453)
}
