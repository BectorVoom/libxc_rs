//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta508 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2001;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2002;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta508(t20945: f64, t20946: f64, t3603: f64, t5284: f64, t5332: f64, t3720: f64, t12866: f64, t17340: f64, t17342: f64, t17693: f64, t17729: f64, t20914: f64, t20917: f64, t20923: f64, t20927: f64, t20929: f64, t20934: f64, t20938: f64, t20941: f64, t3711: f64, t5340: f64, t11249: f64, t6628: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t20947, t20950, t20951, t20952, t20955) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2001(t20945, t20946, t3603, t5284, t5332, t3720, t12866, t17340, t17342, t17693, t17729, t20914, t20917, t20923, t20927, t20929, t20934, t20938, t20941, t3711, t5340);
        let t20956 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2002(t11249, t6628);
    (t20947, t20950, t20951, t20952, t20955, t20956)
}
