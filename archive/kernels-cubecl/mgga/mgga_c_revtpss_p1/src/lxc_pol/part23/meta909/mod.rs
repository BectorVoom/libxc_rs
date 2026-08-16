//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta909 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2919;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2920;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2921;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2922;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta909<F: Float>(t141: F, t77579: F, t930: F, t2908: F, t77584: F, t11341: F, t77564: F, t77568: F, t41294: F, t77573: F, t42731: F, t52011: F, t77513: F, t42518: F, t41307: F, t63276: F, t63278: F, t77507: F, t77509: F, t23495: F, t698: F, t52018: F, t41361: F, t51974: F, t51978: F, t63320: F, t77515: F, t77518: F, t77521: F, t77527: F, t77531: F, t77535: F, t41329: F, t52082: F, t77499: F, t77503: F, t77505: F, t77539: F, t77543: F, t77547: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t77712, t77715, t77718, t77721, t77724, t77727) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2919::<F>(t141, t77579, t930, t2908, t77584, t11341, t77564, t77568, t41294, t77573, t42731, t52011, t77513);
        let (t77730, t77732) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2920::<F>(t42518, t52011, t77513, t41307, t63276, t63278, t77507, t77509, t77712, t77715, t77718, t77721, t77724, t77727);
        let (t77736, t77739, t77747) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2921::<F>(t23495, t698, t52011, t52018, t77513, t41361, t51974, t51978, t63320, t77515, t77518, t77521, t77527, t77531, t77535);
        let t77778 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2922::<F>(t41329, t41361, t51978, t52082, t63276, t63278, t77499, t77503, t77505, t77507, t77509, t77515, t77518, t77521, t77527, t77531, t77535, t77539, t77543, t77547);
    (t77712, t77715, t77718, t77721, t77724, t77727, t77730, t77732, t77736, t77739, t77747, t77778)
}
