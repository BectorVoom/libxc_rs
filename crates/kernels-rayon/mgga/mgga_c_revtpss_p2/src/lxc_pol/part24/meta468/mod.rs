//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta468 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1444;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1445;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta468(t10722: f64, t5993: f64, t40593: f64, t6037: f64, t124: f64, t6016: f64, t10744: f64, t18418: f64, t808: f64, t10886: f64, t18599: f64, t1544: f64, t1559: f64, t40834: f64, t854: f64, t18608: f64, t18352: f64, t2710: f64, t2713: f64, t6030: f64, t18419: f64, t9775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61677, t61699, t61715, t61797, t61833, t61837) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1444(t10722, t5993, t40593, t6037, t124, t6016, t10744, t18418, t808, t10886, t18599, t1544, t1559);
        let (t61839, t61877, t61888, t61890, t61892) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1445(t40834, t61837, t854, t10886, t18608, t808, t18352, t2710, t2713, t10722, t6030, t18419, t9775);
    (t61677, t61699, t61715, t61797, t61833, t61837, t61839, t61877, t61888, t61890, t61892)
}
