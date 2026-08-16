//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1240/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1240(t151: f64, t2124: f64, t2126: f64, t2168: f64, t22787: f64, t23040: f64, t23270: f64, t3467: f64, t3501: f64, t56082: f64, t56103: f64, t56106: f64, t56107: f64, t56115: f64, t56119: f64, t56127: f64, t56128: f64, t56135: f64, t56136: f64, t56144: f64, t56145: f64, t56149: f64, t56404: f64, t675: f64, t9955: f64, t9961: f64) -> f64 {
    let t56457 = 0.20863587575493018851e1_f64 * t23040 * t675 * t56404 * t22787 - 0.10882232163006666614e1_f64 * t3501 * t56082 - 0.417271751509860377e1_f64 * t3467 * t2126 * t56106 - 0.31295381363239528276e1_f64 * t9961 * t151 * t56135 + 0.72548214420044444093e1_f64 * t2168 * t56145 - 0.21764464326013333228e1_f64 * t3501 * t56107 - 0.10882232163006666614e1_f64 * t9955 * t56136 + 0.24182738140014814697e0_f64 * t2168 * t56115 - 0.90685268025055555116e-1_f64 * t2168 * t56149 + 0.5441116081503333307e1_f64 * t3501 * t56103 + 0.36274107210022222046e0_f64 * t2168 * t56119 + 0.10431793787746509425e1_f64 * t2124 * t2126 * t56127 + 0.36274107210022222046e0_f64 * t2168 * t56128 + 0.83454350301972075403e1_f64 * t2124 * t23270 * t56144;
    t56457
}
