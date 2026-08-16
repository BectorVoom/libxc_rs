//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta213 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk994;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk995;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk996;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta213(t45: f64, t10439: f64, t10440: f64, t2401: f64, t750: f64, t200: f64, t2375: f64, t606: f64, t10326: f64, t10356: f64, t2258: f64, t78: f64, zeta_threshold: f64, t57: f64, t202: f64, t2382: f64, t81: f64, t150: f64, t190: f64, t80: f64, t633: f64, t766: f64, t83: f64, t637: f64, t770: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10442, t10444, t10446, t10449, t10455) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk994(t45, t10439, t10440, t2401, t750, t200, t2375, t606, t10326, t10356, t2258, t78, zeta_threshold);
        let (t10457, t10460, t10467, t10468, t10469, t10472) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk995(t57, t202, t2382, t606, t10326, t10356, t2258, t81, t10455, t150, t190, t80, zeta_threshold);
        let (t10481, t10489) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk996(t45, t57, t10326, t10356, t10472, t2258, t633, t766, t606, t83, t637, t770, zeta_threshold);
    (t10442, t10444, t10446, t10449, t10457, t10460, t10467, t10468, t10469, t10472, t10481, t10489)
}
