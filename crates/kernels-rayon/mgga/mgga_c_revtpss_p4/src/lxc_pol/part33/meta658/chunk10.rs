//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2126/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2126(t106005: f64, t106020: f64, t106028: f64, t106055: f64, t106067: f64, t106078: f64, t106092: f64, t106108: f64, t105974: f64, t105976: f64, t1580: f64, t213: f64, t225: f64, t25322: f64, t257: f64, t6049: f64, t92895: f64, t92905: f64, t98875: f64, t98879: f64, t98881: f64, t98894: f64, t98897: f64, t98907: f64, t98911: f64, t99429: f64) -> (f64, f64) {
    let t106111 = t106005 + t106020 + t106028 + t106055 + t106067 + t106078 + t106092 + t106108;
    let t106116 = 0.17135234354032049604e-2_f64 * t92895 + 0.43368140941025997311e-1_f64 * t105974 - 0.77108554593144223219e-1_f64 * t105976 + 0.48186823267806663678e-3_f64 * t92905 + 0.45699670022203476294e-2_f64 * t98875 - t98879 + t98881 + 0.13170898365871023197e1_f64 * t25322 * t6049 - 0.13170898365871023197e1_f64 * t99429 * t1580 + t98894 - t98897 + t98907 - t98911 + 0.65854491829355115987e0_f64 * t213 * t106111 * t225 * t257;
    (t106111, t106116)
}
