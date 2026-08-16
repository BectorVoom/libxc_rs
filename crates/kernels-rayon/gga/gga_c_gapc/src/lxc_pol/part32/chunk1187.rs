//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1187/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1187(t11322: f64, t611: f64, t9386: f64, t11483: f64, t11485: f64, t1846: f64, t34638: f64, t34641: f64, t34644: f64, t34647: f64, t34651: f64, t34654: f64, t34658: f64, t34661: f64, t34663: f64) -> f64 {
    let t34666 = t611 * t9386 * t11322;
    let t34669 = t1846 * t11483 * t11485;
    let t34671 = -0.21720231316129303386e-4_f64 * t34638 - 0.35979010468099443629e-7_f64 * t34641 + 0.53968515702149165444e-6_f64 * t34644 + 0.4797801045921060808e-7_f64 * t34647 + 0.49166375783284505216e-8_f64 * t34651 + 0.24583187891642252608e-8_f64 * t34654 - 0.32777583855523003478e-8_f64 * t34658 - 0.10860115658064651693e-4_f64 * t34661 - 0.5686343261418565457e-6_f64 * t34663 + 0.27462095132499841011e-4_f64 * t34666 + 0.2318836277704281739e-4_f64 * t34669;
    t34671
}
