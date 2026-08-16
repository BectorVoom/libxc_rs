//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1200/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1200(t102131: f64, t102133: f64, t102135: f64, t102139: f64, t102205: f64, t109417: f64, t109426: f64, t109434: f64, t109437: f64, t115067: f64, t1882: f64, t213: f64, t225: f64, t30247: f64, t543: f64, t561: f64, t6843: f64, t7295: f64, t7301: f64, t8085: f64, t96210: f64, t96218: f64, t96230: f64) -> f64 {
    let t115098 = 0.16463622957338778996e-1_f64 * t109417 + 0.72280234901709995519e-3_f64 * t102131 + 0.51405703062096148814e-2_f64 * t102133 - 0.68549505033305214441e-2_f64 * t102135 - 0.19514881078765566038e-2_f64 * t102139 - 0.21684070470512998656e-1_f64 * t109426 - t96210 + 0.13010442282307799193e1_f64 * t7295 * t7301 * t8085 * t6843 * t543 - t96218 + 0.15421710918628844643e0_f64 * t109434 + 0.13010442282307799193e1_f64 * t7295 * t7301 * t30247 * t1882 * t543 - 0.23132566377943266966e0_f64 * t109437 + t96230 + 0.65854491829355115987e0_f64 * t213 * t115067 * t225 * t561 + 0.13709901006661042888e-1_f64 * t102205;
    t115098
}
