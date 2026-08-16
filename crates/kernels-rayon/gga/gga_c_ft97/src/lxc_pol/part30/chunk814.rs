//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 814/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk814(t34109: f64, t34153: f64, t34196: f64, t34249: f64, t33983: f64, t6223: f64, t193: f64, t1466: f64, t301: f64, t33808: f64, t33994: f64, t33998: f64, t34003: f64, t34008: f64, t34013: f64, t34015: f64, t34017: f64, t34019: f64, t34022: f64, t34025: f64, t34054: f64, t34058: f64, t6210: f64, t6216: f64, t6219: f64, t6225: f64, t7581: f64, t7614: f64, t7684: f64, t830: f64) -> (f64, f64, f64, f64) {
    let t34251 = t34109 + t34153 + t34196 + t34249;
    let t34253 = t33983 * t6223;
    let t34254 = t193 * t34253;
    let t34259 = -t33808 * t6219 / 18.0_f64 + 2.0_f64 * t33994 - t6216 * t33998 / 9.0_f64 - t6216 * t34003 / 18.0_f64 + t6216 * t34008 / 9.0_f64 - t830 * t7684 - 2.0_f64 * t34013 - 4.0_f64 * t34015 + 4.0_f64 * t34017 - 2.0_f64 * t34019 + t1466 * t34022 - 2.0_f64 / 3.0_f64 * t1466 * t34025 - t7581 * t6225 / 3.0_f64 - 2.0_f64 * t34054 - 2.0_f64 / 3.0_f64 * t1466 * t34058 - t301 * t34251 - t1466 * t34254 / 3.0_f64 + t6210 * t7614 / 6.0_f64;
    (t34251, t34253, t34254, t34259)
}
