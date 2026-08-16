//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta513 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1923;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1924;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta513(t1976: f64, t5015: f64, t7160: f64, t3046: f64, t7143: f64, t1032: f64, t1678: f64, t7150: f64, t4742: f64, t7145: f64, t1695: f64, t7135: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27411, t27412, t27415, t27418) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1923(t1976, t5015, t7160, t3046, t7143, t1032, t1678);
        let (t27419, t27422, t27423, t27426) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1924(t27418, t7150, t1976, t4742, t7145, t1695, t7135);
    (t27411, t27412, t27415, t27418, t27419, t27422, t27423, t27426)
}
