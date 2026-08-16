//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta551 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1638;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1639;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1640;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1641;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta551<F: Float>(t6079: F, t1544: F, t1583: F, t18850: F, t1940: F, t198: F, t207: F, t23148: F, t2403: F, t40076: F, t40079: F, t40194: F, t40198: F, t41154: F, t4541: F, t4546: F, t5966: F, t765: F, t77357: F, t77373: F, t87543: F, t87676: F, t87677: F, t87678: F, t87679: F, t87302: F, t87316: F, t87931: F, t87942: F, t87951: F, t87952: F, t87966: F, t6206: F, t6226: F, t981: F, t19133: F, t19303: F, t6189: F, t41235: F, t41238: F, t11509: F, t41224: F, t6141: F, t2874: F, t935: F, t2924: F, t2926: F, t6110: F, t63677: F) -> (F, F, F, F, F, F, F, F, F) {
        let t87987 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1638::<F>(t6079, t1544, t1583, t18850, t1940, t198, t207, t23148, t2403, t40076, t40079, t40194, t40198, t41154, t4541, t4546, t5966, t765, t77357, t77373, t87543, t87676, t87677, t87678, t87679);
        let (t87990, t88004, t88007) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1639::<F>(t87302, t87316, t87931, t87942, t87951, t87952, t87966, t87987, t6206, t6226, t981, t19133, t19303);
        let t88008 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1640::<F>(t6189);
        let (t88012, t88016, t88023, t88026, t88028) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1641::<F>(t41235, t41238, t88008, t981, t11509, t41224, t6141, t2874, t935, t2924, t2926, t6110, t63677);
    (t87990, t88004, t88007, t88008, t88012, t88016, t88023, t88026, t88028)
}
