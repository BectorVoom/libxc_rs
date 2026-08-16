//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta573 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1944;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1945;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta573(t1949: f64, t5844: f64, t5838: f64, t1599: f64, t7614: f64, t23678: f64, t5928: f64, t23677: f64, t23604: f64, t23603: f64, t28596: f64, t3188: f64, t1058: f64, t1610: f64, t1953: f64, t23327: f64, t23601: f64, t23633: f64, t25530: f64, t25563: f64, t28638: f64, t28642: f64, t28648: f64, t28653: f64, t3186: f64, t5903: f64, t6687: f64, t7622: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28657, t28660, t28663, t28666, t28667, t28670, t28671, t28674) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1944(t1949, t5844, t5838, t1599, t7614, t23678, t5928, t23677, t23604, t23603, t28596, t3188);
        let t28677 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1945(t1058, t1610, t1953, t23327, t23601, t23633, t25530, t25563, t28638, t28642, t28648, t28653, t28657, t28660, t28663, t28667, t28671, t28674, t3186, t5903, t6687, t7622);
    (t28657, t28660, t28663, t28666, t28667, t28670, t28671, t28674, t28677)
}
