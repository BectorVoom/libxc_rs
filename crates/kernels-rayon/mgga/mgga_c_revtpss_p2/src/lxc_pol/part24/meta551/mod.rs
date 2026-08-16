//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta551 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1638;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1639;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1640;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1641;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta551(t6079: f64, t1544: f64, t1583: f64, t18850: f64, t1940: f64, t198: f64, t207: f64, t23148: f64, t2403: f64, t40076: f64, t40079: f64, t40194: f64, t40198: f64, t41154: f64, t4541: f64, t4546: f64, t5966: f64, t765: f64, t77357: f64, t77373: f64, t87543: f64, t87676: f64, t87677: f64, t87678: f64, t87679: f64, t87302: f64, t87316: f64, t87931: f64, t87942: f64, t87951: f64, t87952: f64, t87966: f64, t6206: f64, t6226: f64, t981: f64, t19133: f64, t19303: f64, t6189: f64, t41235: f64, t41238: f64, t11509: f64, t41224: f64, t6141: f64, t2874: f64, t935: f64, t2924: f64, t2926: f64, t6110: f64, t63677: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t87987 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1638(t6079, t1544, t1583, t18850, t1940, t198, t207, t23148, t2403, t40076, t40079, t40194, t40198, t41154, t4541, t4546, t5966, t765, t77357, t77373, t87543, t87676, t87677, t87678, t87679);
        let (t87990, t88004, t88007) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1639(t87302, t87316, t87931, t87942, t87951, t87952, t87966, t87987, t6206, t6226, t981, t19133, t19303);
        let t88008 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1640(t6189);
        let (t88012, t88016, t88023, t88026, t88028) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1641(t41235, t41238, t88008, t981, t11509, t41224, t6141, t2874, t935, t2924, t2926, t6110, t63677);
    (t87990, t88004, t88007, t88008, t88012, t88016, t88023, t88026, t88028)
}
