//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 891/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk891(t7064: f64, t7069: f64, t8878: f64, t13212: f64, t7129: f64, t40693: f64, t40696: f64, t40699: f64, t40612: f64, t40614: f64, t40620: f64, t40622: f64, t40627: f64, t40630: f64, t40632: f64, t40634: f64, t471: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43042 = t7064 * t8878 * t7069;
    let t43043 = 0.1922631557535556071e-2_f64 * t43042;
    let t43049 = 0.23071578690426672851e-1_f64 * t7129 * t13212;
    let t43053 = 0.64087718584518535698e-3_f64 * t40693;
    let t43054 = 0.64087718584518535698e-3_f64 * t40696;
    let t43055 = 0.64087718584518535698e-3_f64 * t40699;
    let t43069 = (21.0_f64 / 512.0_f64 * t40612 + 357.0_f64 / 16384.0_f64 * t40614 - 189.0_f64 / 262144.0_f64 * t40620 + 189.0_f64 / 0.16777216e8_f64 * t40622 - 63.0_f64 / 0.16777216e8_f64 * t40627 + 63.0_f64 / 262144.0_f64 * t40630 - 119.0_f64 / 16384.0_f64 * t40632 - 7.0_f64 / 512.0_f64 * t40634) * t471;
    (t43043, t43049, t43053, t43054, t43055, t43069)
}
