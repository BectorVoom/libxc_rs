//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 743/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk743(t1302: f64, t32174: f64, t1303: f64, t22736: f64, t22796: f64, t3066: f64, t32125: f64, t32129: f64, t32133: f64, t32138: f64, t32140: f64, t32141: f64, t32146: f64, t32148: f64, t32153: f64, t32156: f64, t32161: f64, t32164: f64, t32169: f64, t32170: f64, t429: f64, t5533: f64, t5587: f64, t7172: f64) -> (f64, f64) {
    let t32175 = t32174 * t1302;
    let t32178 = -0.76612330055555555556e-1_f64 * t32125 * t1303 - 0.76612330055555555556e-1_f64 * t32129 * t1303 - 0.22979081259345929704e-6_f64 * t22736 * t32133 * t3066 + 0.11738898233082762228e-1_f64 * t32138 * t32140 * t32141 + 0.89080607335887169333e-3_f64 * t32146 * t32148 - 0.39601100101559655353e-5_f64 * t22796 * t32153 + 4.0_f64 * t32156 * t5533 - 2.0_f64 * t7172 * t429 + 0.42300125954037691564e-4_f64 * t32161 * t32164 - 0.17608347349624143343e-1_f64 * t32169 * t32140 * t32170 - 0.27246626553445399075e-2_f64 * t5587 * t32175;
    (t32175, t32178)
}
