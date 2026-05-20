//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta920 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2969;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2970;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta920<F: Float>(t19147: F, t4719: F, t23694: F, t2986: F, t974: F, t981: F, t77863: F, t964: F, t973: F, t19468: F, t19134: F, t78094: F, t78096: F, t78099: F, t78154: F, t78192: F, t78195: F, t78201: F, t78203: F, t78206: F, t78246: F, t78248: F, t78251: F, t78254: F, t78472: F, t78474: F, t4711: F, t64504: F, t23811: F, t300: F, t983: F, t52238: F, t78423: F, t18898: F, t52459: F, t15258: F, t19133: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t78686, t78690, t78694, t78696, t78698, t78699) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2969::<F>(t19147, t4719, t23694, t2986, t974, t981, t77863, t964, t973, t19468, t19134, t78094, t78096, t78099, t78154, t78192, t78195, t78201, t78203, t78206, t78246, t78248, t78251, t78254, t78472, t78474);
        let (t78703, t78706, t78709, t78712, t78715) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2970::<F>(t4711, t64504, t981, t23811, t300, t983, t52238, t78423, t18898, t52459, t15258, t19133);
    (t78686, t78690, t78694, t78696, t78698, t78699, t78703, t78706, t78709, t78712, t78715)
}
