//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1432/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1432(t1873: f64, t45814: f64, t12739: f64, t6534: f64, t22479: f64, t5113: f64, t22461: f64, t2363: f64, t26103: f64, t6517: f64, t671: f64, t83853: f64, t83889: f64, t83935: f64, t83946: f64, t83948: f64, t83952: f64, t83956: f64, t83958: f64, t83960: f64, t83962: f64, t9416: f64) -> f64 {
    let t83964 = 2.0_f64 * t45814 * t1873;
    let t83966 = 6.0_f64 * t12739 * t6534;
    let t83968 = 6.0_f64 * t5113 * t22479;
    let t83969 = 6.0_f64 * t22461 * t2363 + 6.0_f64 * t2363 * t26103 + 2.0_f64 * t6517 * t9416 + 6.0_f64 * t671 * t83935 + t83853 + 6.0_f64 * t83889 + t83946 + t83948 + t83952 + t83956 + t83958 + t83960 + t83962 + t83964 + t83966 + t83968;
    t83969
}
