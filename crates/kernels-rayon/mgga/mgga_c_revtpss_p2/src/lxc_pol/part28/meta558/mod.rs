//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta558 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2013;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2014;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta558(t2487: f64, t93034: f64, t10832: f64, t25245: f64, t25266: f64, t2648: f64, t2681: f64, t7036: f64, t820: f64, t839: f64, t25260: f64, t843: f64, t2726: f64, t10841: f64, t10867: f64, t64: f64, t7043: f64, t857: f64, t25222: f64, t2656: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93035, t93043, t93045, t93048, t93049, t93054) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2013(t2487, t93034, t10832, t25245, t25266, t2648, t2681, t7036, t820, t839, t25260, t843);
        let (t93055, t93058, t93060, t93066, t93067, t93069) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2014(t2726, t93054, t10841, t25245, t10867, t64, t2681, t7043, t820, t857, t25222, t2656);
    (t93035, t93043, t93045, t93048, t93049, t93055, t93058, t93060, t93066, t93067, t93069)
}
