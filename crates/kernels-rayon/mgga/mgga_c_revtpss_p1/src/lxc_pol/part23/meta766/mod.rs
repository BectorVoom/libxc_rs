//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta766 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2566;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta766(t487: f64, t56331: f64, t56176: f64, t56183: f64, t56228: f64, t12627: f64, t1811: f64, t1269: f64, t17306: f64, t3565: f64, t5215: f64, t3566: f64, t5412: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56332, t56343, t56345, t56360, t56393, t56416, t56447, t56462, t56587, t56588, t56607) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2566(t487, t56331, t56176, t56183, t56228, t12627, t1811, t1269, t17306, t3565, t5215, t3566, t5412);
    (t56332, t56343, t56345, t56360, t56393, t56416, t56447, t56462, t56587, t56588, t56607)
}
