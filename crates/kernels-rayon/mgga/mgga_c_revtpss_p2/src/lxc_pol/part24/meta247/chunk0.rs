//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1010/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1010(t2689: f64, t4372: f64, t4354: f64, t9775: f64, t10722: f64, t1565: f64, t10868: f64, t241: f64, t820: f64, t2719: f64, t844: f64, t2482: f64, t814: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14846 = t2689 * t4372;
    let t14850 = t9775 * t4354;
    let t14866 = t10722 * t1565;
    let t14894 = t820 * t10868 * t241;
    let t14923 = t820 * t2719 * t844;
    let t14931 = t2482 * t2719 * t814;
    (t14846, t14850, t14866, t14894, t14923, t14931)
}
