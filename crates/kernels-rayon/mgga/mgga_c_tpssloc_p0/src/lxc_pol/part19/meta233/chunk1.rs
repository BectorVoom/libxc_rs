//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 941/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk941(t1055: f64, t11084: f64, t10160: f64, t10167: f64, t10170: f64, t10182: f64, t1052: f64, t1066: f64, t11008: f64, t11010: f64, t11013: f64, t11016: f64, t11018: f64, t3026: f64, t3169: f64, t3176: f64, t3207: f64, t388: f64) -> (f64, f64) {
    let t11085 = t1055 * t11084;
    let t11087 = -6.0_f64 * t10160 * t1066 - 6.0_f64 * t10167 * t1052 - 3.0_f64 * t10170 * t1066 + 6.0_f64 * t10182 * t1052 - t1052 * t11085 - 3.0_f64 * t1066 * t11010 + t11008 * t388 + 3.0_f64 * t11013 * t388 + t11016 * t388 + 3.0_f64 * t11018 * t388 + 6.0_f64 * t3026 * t3176 - 3.0_f64 * t3026 * t3207 + 6.0_f64 * t3169 * t3176 - 3.0_f64 * t3169 * t3207;
    (t11085, t11087)
}
