//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta329 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1121;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1122;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta329(t14616: f64, t757: f64, t1544: f64, t2475: f64, t124: f64, t1558: f64, t10779: f64, t2749: f64, t10777: f64, t125: f64, t4423: f64, t136: f64, t243: f64, t220: f64, t837: f64, t1548: f64, t10811: f64, t4447: f64, t10815: f64, t1561: f64, t2741: f64, t4426: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14618, t14648, t14671, t14675, t14676, t14685) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1121(t14616, t757, t1544, t2475, t124, t1558, t10779, t2749, t10777, t125, t4423, t136, t243);
        let (t14686, t14690, t14703, t14705, t14712, t14715) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1122(t14685, t220, t14671, t837, t10777, t10779, t1548, t10811, t4447, t10815, t1561, t2741, t4426);
    (t14618, t14648, t14671, t14675, t14676, t14686, t14690, t14703, t14705, t14712, t14715)
}
