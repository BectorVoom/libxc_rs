//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 980/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk980(t10339: f64, t10355: f64, t18826: f64, t18831: f64, t18834: f64, t19151: f64, t19155: f64, t19163: f64, t19167: f64, t19169: f64, t19174: f64, t19178: f64, t19181: f64, t19184: f64, t19189: f64, t19192: f64, t2014: f64, t2394: f64, t2710: f64, t274: f64, t4068: f64, t4069: f64, t807: f64, t8948: f64, t8963: f64, t9609: f64) -> f64 {
    let t19201 = 0.17557713923258613e0_f64 * t18826 * t4069 - 0.23410285231011484e0_f64 * t4068 * t19151 + 0.33205381699090447729e-3_f64 * t8948 * t19155 - 0.11705142615505742e0_f64 * t18831 * t4069 + 0.23410285231011484e0_f64 * t18834 * t274 - 0.26564305359272358183e-2_f64 * t2014 * t19163 - t19167 - 0.8854768453090786061e-3_f64 * t8963 * t19169 - 0.53128610718544716366e-2_f64 * t2014 * t19174 + 0.72343824494974941953e-3_f64 * t8963 * t19178 - 0.5116527820486904976e-1_f64 * t10339 * t19181 + 0.639565977560863122e-1_f64 * t2710 * t19184 - 0.25159457085530922489e-1_f64 * t9609 * t19181 + 0.319782988780431561e-1_f64 * t2710 * t19189 - 0.532971647967385935e-1_f64 * t807 * t19192 + 0.13977476158628290272e-1_f64 * t2394 * t19189 + 0.27954952317256580544e-1_f64 * t2394 * t19184 - 0.91641760171536135284e-3_f64 * t10355 * t19181;
    t19201
}
