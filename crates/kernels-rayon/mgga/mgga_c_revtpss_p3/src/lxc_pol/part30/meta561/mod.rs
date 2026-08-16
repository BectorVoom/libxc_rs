//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta561 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2005;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2006;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta561(t10836: f64, t25227: f64, t2661: f64, t2482: f64, t596: f64, t7036: f64, t2487: f64, t10832: f64, t25245: f64, t25266: f64, t2648: f64, t2681: f64, t820: f64, t839: f64, t25260: f64, t843: f64, t2726: f64, t10841: f64, t10867: f64, t64: f64, t7043: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93031, t93034, t93035, t93043, t93045, t93048) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2005(t10836, t25227, t2661, t2482, t596, t7036, t2487, t10832, t25245, t25266, t2648, t2681, t820);
        let (t93049, t93055, t93058, t93060, t93066) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2006(t839, t93048, t25260, t820, t843, t2726, t10841, t25245, t10867, t64, t2681, t7043);
    (t93031, t93034, t93035, t93043, t93045, t93048, t93049, t93055, t93058, t93060, t93066)
}
