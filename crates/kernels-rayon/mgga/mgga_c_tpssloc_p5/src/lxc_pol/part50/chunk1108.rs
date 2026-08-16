//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1108/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1108(t1635: f64, t30912: f64, t30915: f64, t32987: f64, t32993: f64, t32998: f64, t33001: f64, t33005: f64, t33007: f64, t388: f64, t4557: f64, t4660: f64, t6687: f64, t6771: f64, t7600: f64, t7625: f64, t8397: f64, t8407: f64) -> f64 {
    let t33012 = 2.0_f64 * t4557 * t8397 + 0.54831135561607547883e-2_f64 * t6687 * t32987 + 4.0_f64 * t6771 * t7600 - 0.16449340668482264365e-1_f64 * t6687 * t32993 + 2.0_f64 * t4660 * t8397 - 0.16449340668482264365e-1_f64 * t6687 * t32998 - 0.16449340668482264365e-1_f64 * t6687 * t33001 - t4660 * t8407 + t33005 * t388 + t33007 * t388 - t30915 * t1635 - 2.0_f64 * t6771 * t7625 - t30912;
    t33012
}
