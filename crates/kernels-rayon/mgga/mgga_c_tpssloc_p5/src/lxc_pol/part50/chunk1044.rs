//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1044/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1044(t30843: f64, t349: f64, t1052: f64, t1920: f64, t23327: f64, t3026: f64, t30778: f64, t30783: f64, t30789: f64, t30793: f64, t30798: f64, t30801: f64, t30805: f64, t30808: f64, t3169: f64, t388: f64, t6680: f64, t6687: f64, t6771: f64, t6776: f64, t6816: f64, t8377: f64, t8397: f64, t8407: f64) -> (f64, f64) {
    let t30844 = t349 * t30843;
    let t30853 = 2.0_f64 * t1052 * t30778 - 0.54831135561607547883e-2_f64 * t23327 * t30783 + 4.0_f64 * t6771 * t6776 + 0.54831135561607547883e-2_f64 * t6687 * t30789 + 4.0_f64 * t1052 * t30793 + t30798 + 0.16449340668482264365e-1_f64 * t1920 * t30801 - 6.0_f64 * t1052 * t30805 + t30808 * t388 + t30844 * t388 - 0.43864908449286038307e-1_f64 * t6680 * t8377 - t3026 * t8407 + 2.0_f64 * t3169 * t8397 - 2.0_f64 * t6771 * t6816;
    (t30844, t30853)
}
