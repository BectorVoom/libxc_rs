//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1843/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1843(t15909: f64, t12044: f64, t12046: f64, t12048: f64, t12053: f64, t12055: f64, t12057: f64, t12059: f64, t12087: f64, t20372: f64, t20398: f64, t9780: f64, t9789: f64) -> (f64, f64) {
    let t20520 = 0.32530743900905219526e-1_f64 * t15909;
    let t20521 = -t20372 + t9780 + t20398 + t20520 - t12044 - t12046 - t12048 + t12053 - t12055 - t12057 - t12059 - t9789 + t12087;
    (t20520, t20521)
}
