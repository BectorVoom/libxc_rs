//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 738/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk738<F: Float>(t3: F, t3931: F, t112: F, t1395: F, t111: F, t576: F, t1401: F, t2319: F, t2363: F, t577: F, t671: F, t2218: F, t2221: F, t2225: F, t2232: F) -> (F, F, F, F, F) {
    let t3932 = t3 * t3931;
    let t3938 = t1395 * t112;
    let t3941 = t576 * t111;
    let t3946 = F::cast_from(0.45e1_f64) * t3931 * t577 + F::cast_from(27.0_f64) * t3938 * t671 + F::cast_from(27.0_f64) * t3941 * t2319 + F::cast_from(0.135e2_f64) * t1401 * t2363;
    let t3951 = -t2218 - F::cast_from(0.78e0_f64) * t2221 - F::cast_from(0.578e2_f64) * t2225 + t2232;
    (t3932, t3938, t3941, t3946, t3951)
}
