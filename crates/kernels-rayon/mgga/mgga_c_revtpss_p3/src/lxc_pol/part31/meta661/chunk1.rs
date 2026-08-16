//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2240/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2240(t109118: f64, t2014: f64, t7237: f64, t28167: f64, t35669: f64, t5627: f64, t29996: f64, t7235: f64, t22483: f64, t7312: f64, t109078: f64, t109081: f64, t109087: f64, t109090: f64, t109092: f64, t109095: f64, t109099: f64, t109103: f64, t109107: f64, t109110: f64, t109112: f64, t109117: f64, t1843: f64, t1911: f64, t28160: f64, t28230: f64, t5517: f64, t7725: f64) -> f64 {
    let t109121 = 3.0_f64 * t2014 * t7237 * t109118;
    let t109124 = 12.0_f64 * t28167 * t35669 * t5627;
    let t109126 = 2.0_f64 * t7235 * t29996;
    let t109128 = t2014 * t7312 * t22483;
    let t109129 = -2.0_f64 * t1843 * t28160 + 2.0_f64 * t1911 * t28230 - 2.0_f64 * t5517 * t7725 + t109078 - t109081 + t109087 + t109090 - t109092 - t109095 - t109099 + t109103 - t109107 + t109110 + t109112 - t109117 + t109121 + t109124 - t109126 - t109128;
    t109129
}
