//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta623 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2102;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2103;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta623(t39063: f64, t6489: f64, t22573: f64, t6875: f64, t22947: f64, t532: f64, t111: f64, t22558: f64, t7002: f64, t112: f64, t23862: f64, t1395: f64, t7020: f64, t26555: f64, t576: f64, t1858: f64, t2029: f64, t5363: f64, t1851: f64, t16507: f64, t16546: f64, t1852: f64, t2023: f64, t23863: f64, t23901: f64, t3946: f64, t5381: f64, t7003: f64, t7759: f64, t80593: f64, t80597: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t83830, t83886, t83929, t83935, t83980, t84004, t84024) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2102(t39063, t6489, t22573, t6875, t22947, t532, t111, t22558, t7002, t112, t23862, t1395, t7020);
        let t86580 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2103(t26555, t576, t1858, t7002, t2029, t5363, t1851, t7020, t16507, t16546, t1852, t2023, t23863, t23901, t3946, t5381, t7003, t7759, t80593, t80597, t84024);
    (t83830, t83886, t83929, t83935, t83980, t84004, t86580)
}
