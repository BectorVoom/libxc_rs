//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1213/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1213(t23384: f64, t32973: f64, t1052: f64, t113201: f64, t113219: f64, t113261: f64, t113601: f64, t113608: f64, t113611: f64, t1599: f64, t1635: f64, t1945: f64, t23346: f64, t23372: f64, t25452: f64, t25712: f64, t25742: f64, t25757: f64, t25758: f64, t30778: f64, t30793: f64, t30915: f64, t3174: f64, t32981: f64, t33001: f64, t343: f64, t4557: f64, t4660: f64, t4665: f64, t6687: f64, t6690: f64, t6815: f64, t7553: f64, t7624: f64, t7625: f64) -> f64 {
    let t119559 = t23384 * t32973;
    let t119571 = 0.14621636149762012769e-1_f64 * t113601 + 4.0_f64 * t1052 * t3174 * t6815 * t7624 + 0.54831135561607547883e-2_f64 * t6687 * t113261 * t7553 - 2.0_f64 * t23372 * t7625 + 4.0_f64 * t4660 * t30793 + 0.16449340668482264365e-1_f64 * t6687 * t1599 * t113201 + 2.0_f64 * t4557 * t30778 - 0.87729816898572076613e-1_f64 * t23346 * t32981 - 0.54831135561607547883e-2_f64 * t113608 + 0.43864908449286038307e-1_f64 * t23346 * t33001 + 0.54831135561607547883e-2_f64 * t113611 - 0.16449340668482264365e-1_f64 * t6687 * t25712 * t343 * t1945 * t6690 - t113219 * t1635 + 0.54831135561607547883e-2_f64 * t119559 - 0.43864908449286038307e-1_f64 * t23346 * t32973 + 2.0_f64 * t30915 * t4665 - 12.0_f64 * t25757 * t25758 * t25452 - 12.0_f64 * t25757 * t25758 * t25742;
    t119571
}
