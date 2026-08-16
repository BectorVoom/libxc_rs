//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1054/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1054(t5456: f64, t8828: f64, t116075: f64, t117477: f64, t122988: f64, t123001: f64, t124803: f64, t126091: f64, t126103: f64, t129084: f64, t31860: f64, t32338: f64, t33669: f64, t33677: f64, t34122: f64, t34132: f64, t5389: f64, t5392: f64, t5441: f64, t5445: f64, t63: f64, t7246: f64, t8513: f64, t8663: f64, t8824: f64, t8825: f64) -> (f64, f64) {
    let t130377 = t8828 * t5456;
    let t130412 = -40.0_f64 / 27.0_f64 * t124803 + 5.0_f64 / 6.0_f64 * t123001 * t34122 + 5.0_f64 / 12.0_f64 * t31860 * t8513 * t8824 * t5445 - 5.0_f64 / 9.0_f64 * t126091 * t117477 - 5.0_f64 / 18.0_f64 * t33677 * t34132 - 5.0_f64 / 36.0_f64 * t8663 * t8513 * t32338 * t5441 - 35.0_f64 / 12.0_f64 * t116075 * t8513 * t8824 * t5389 + 5.0_f64 / 18.0_f64 * t7246 * t8513 * t8824 * t5392 + 5.0_f64 / 6.0_f64 * t122988 * t34122 - 5.0_f64 / 18.0_f64 * t33669 * t34132 - 5.0_f64 / 36.0_f64 * t8663 * t8513 * t126103 * t63 - 5.0_f64 / 72.0_f64 * t129084 * t8825;
    (t130377, t130412)
}
