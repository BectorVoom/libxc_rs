//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 427/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk427(t2255: f64, t772: f64, t2132: f64, t2201: f64, t2209: f64, t2213: f64, t2217: f64, t2221: f64, t2225: f64, t2229: f64, t2234: f64, t2238: f64, t2239: f64, t2242: f64, t2246: f64, t2253: f64, t771: f64, t794: f64, t799: f64, t802: f64, t828: f64, t832: f64) -> f64 {
    let t2256 = t772 * t2255;
    let t2259 = t2132 + 0.2740028945738165176e-4_f64 * t828 * t2201 + 0.39958755458681575483e-5_f64 * t2209 * t2213 - 0.77948343448359322927e-4_f64 * t2217 * t802 - 0.91334298191272172533e-4_f64 * t2221 * t832 + 0.33406432906439709826e-4_f64 * t2225 * t802 + 0.2740028945738165176e-4_f64 * t2229 * t832 + 0.56366309740899397906e-3_f64 * t771 * t2234 + 0.18788769913633132635e-4_f64 * t2238 * t2239 + 0.18788769913633132635e-4_f64 * t794 * t2242 + 0.33406432906439709826e-4_f64 * t799 * t2246 + 0.56366309740899397906e-3_f64 * t2253 * t2256;
    t2259
}
