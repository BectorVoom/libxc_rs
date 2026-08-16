//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta723 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2368;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2369;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta723(t21160: f64, t699: f64, t21167: f64, t47705: f64, t47707: f64, t48103: f64, t49139: f64, t49144: f64, t68442: f64, t68444: f64, t68446: f64, t68448: f64, t16558: f64, t4342: f64, t136: f64, t908: f64, t17156: f64, t3966: f64, t2826: f64, t13527: f64, t5398: f64, t4337: f64, t20234: f64, t41666: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t68452, t68454, t68457) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2368(t21160, t699, t21167, t47705, t47707, t48103, t49139, t49144, t68442, t68444, t68446, t68448);
        let (t68458, t68460, t68462, t68464, t68466, t68468, t68470, t68472, t68477) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2369(t16558, t4342, t136, t908, t17156, t3966, t2826, t13527, t5398, t4337, t20234, t41666, t607);
    (t68452, t68454, t68457, t68458, t68460, t68462, t68464, t68466, t68468, t68470, t68472, t68477)
}
