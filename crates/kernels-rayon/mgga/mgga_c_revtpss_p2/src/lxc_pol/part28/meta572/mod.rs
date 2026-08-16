//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta572 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2034;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2035;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta572(t3058: f64, t8521: f64, t7135: f64, t989: f64, t25625: f64, t7166: f64, t11213: f64, t1976: f64, t11711: f64, t25517: f64, t11865: f64, t25516: f64, t11874: f64, t27492: f64, t11988: f64, t7132: f64, t3196: f64, t7131: f64, t11648: f64, t7122: f64, t25512: f64, t3173: f64, t11916: f64, t25509: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93502, t93509, t93521, t93528, t93541, t93543) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2034(t3058, t8521, t7135, t989, t25625, t7166, t11213, t1976, t11711, t25517, t11865, t25516);
        let (t93548, t93555, t93561, t93564, t93570, t93573) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2035(t11874, t27492, t11988, t7132, t3196, t7131, t11648, t7122, t25512, t3173, t11916, t25509);
    (t93502, t93509, t93521, t93528, t93541, t93543, t93548, t93555, t93561, t93564, t93570, t93573)
}
