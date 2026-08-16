//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 419/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk419(t2188: f64, t297: f64, t314: f64, t129: f64, t519: f64, t919: f64, t1179: f64, t2042: f64, t2043: f64, t2046: f64, t2134: f64, t2155: f64, t2160: f64, t2161: f64, t2165: f64, t2166: f64, t284: f64, t316: f64, t731: f64, t763: f64, t821: f64) -> (f64, f64) {
    let t2189 = t2188 * t297;
    let t2190 = t2189 * t314;
    let t2191 = t129 * t2190;
    let t2196 = t519 * t919;
    let t2199 = t2042 + 0.1252584660908875509e-2_f64 * t2043 * t316 - 0.93943849568165663176e-3_f64 * t2046 * t316 - 0.93943849568165663176e-3_f64 * t731 * t821 + 0.28183154870449698953e-3_f64 * t2155 * t316 - 0.11135477635479903275e-5_f64 * t2160 * t2161 + 0.4871771465522457683e-5_f64 * t2165 * t2166 + 0.28183154870449698953e-3_f64 * t284 * t2191 + 0.56366309740899397906e-3_f64 * t763 * t821 - t1179 + t2134 - 0.2740028945738165176e-5_f64 * t2165 * t2196;
    (t2190, t2199)
}
