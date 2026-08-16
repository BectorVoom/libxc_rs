//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta880 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2788;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2789;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta880(t1882: f64, t2482: f64, t4104: f64, t5767: f64, t1892: f64, t5658: f64, t14230: f64, t2782: f64, t48083: f64, t4086: f64, t543: f64, t10073: f64, t22365: f64, t14141: f64, t14143: f64, t676: f64, t22252: f64, t555: f64, t1419: f64, t6843: f64, t14224: f64, t14238: f64, t6861: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74908, t74935, t74943, t74945) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2788(t1882, t2482, t4104, t5767, t1892, t5658, t14230, t2782, t48083, t4086, t543, t10073, t22365);
        let (t74949, t74965, t74973, t74979, t74982) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2789(t14141, t14143, t5658, t676, t22252, t555, t1419, t6843, t14224, t14238, t2782, t6861);
    (t74908, t74935, t74943, t74945, t74949, t74965, t74973, t74979, t74982)
}
