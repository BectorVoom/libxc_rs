//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1383/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1383(t13133: f64, t13220: f64, t13554: f64, t1799: f64, t18627: f64, t19305: f64, t19656: f64, t3493: f64, t41905: f64, t42336: f64, t42719: f64, t5801: f64, t5815: f64, t6234: f64, t6323: f64, t65094: f64, t65097: f64, t65956: f64, t7798: f64) -> f64 {
    let t67586 = 4.0_f64 * t13133 * t5815 + 2.0_f64 * t13220 * t5801 + 4.0_f64 * t13554 * t5815 + 2.0_f64 * t1799 * t41905 + 2.0_f64 * t1799 * t42336 + 4.0_f64 * t1799 * t42719 + 2.0_f64 * t1799 * t65094 + 4.0_f64 * t1799 * t65097 + 2.0_f64 * t1799 * t65956 + 2.0_f64 * t18627 * t3493 + 2.0_f64 * t18627 * t6234 + 4.0_f64 * t19305 * t5815 + 4.0_f64 * t19656 * t5815 + 2.0_f64 * t6323 * t7798;
    t67586
}
