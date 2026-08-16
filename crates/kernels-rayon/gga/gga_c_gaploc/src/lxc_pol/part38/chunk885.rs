//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 885/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk885(t45108: f64, t13548: f64, t731: f64, t11832: f64, t22090: f64, t2508: f64, t7291: f64, t13498: f64, t13539: f64, t1897: f64, t1901: f64, t43290: f64, t43326: f64, t43330: f64, t45072: f64, t45077: f64, t45079: f64, t45083: f64, t45086: f64, t45090: f64, t45091: f64, t45101: f64, t45104: f64, t45107: f64, t7137: f64) -> f64 {
    let t45109 = 0.32043859292259267849e-3_f64 * t45108;
    let t45110 = t731 * t13548;
    let t45111 = 0.42725145723012357132e-3_f64 * t45110;
    let t45115 = 0.1845726295234133828e0_f64 * t2508 * t22090 * t11832 * t7291;
    let t45116 = -t45072 - 0.61524209841137794269e-1_f64 * t7137 * t13498 - t45077 + t45079 + 0.41016139894091862845e-1_f64 * t7137 * t13539 + t45083 + t45086 + t45090 + 0.76905262301422242837e-2_f64 * t1897 * t1901 * t45091 - 0.17090058289204942853e-2_f64 * t43290 - 0.1281754371690370714e-2_f64 * t43326 - 0.1281754371690370714e-2_f64 * t43330 + t45101 + t45104 - t45107 + t45109 - t45111 + t45115;
    t45116
}
