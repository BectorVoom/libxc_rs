//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1235/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1235(t119: f64, t1264: f64, t150: f64, t1620: f64, t187: f64, t2146: f64, t2222: f64, t2394: f64, t31965: f64, t32124: f64, t33180: f64, t33185: f64, t33198: f64, t36547: f64, t38001: f64, t38241: f64, t38251: f64, t38256: f64, t38259: f64, t5332: f64, t7912: f64, t8004: f64, t8306: f64, t8316: f64, t9145: f64, t9165: f64) -> f64 {
    let t38270 = 0.17347256376410398924e1_f64 * t33180 + t33185 + t38241 - 0.26020884564615598386e1_f64 * t2146 * t8004 * t2394 * t1264 - 0.65854491829355115987e0_f64 * t2222 * t5332 + 0.8673628188205199462e0_f64 * t7912 * t9145 - t38251 - 0.17347256376410398924e1_f64 * t31965 * t9165 + t38256 - t38259 + 0.65854491829355115987e0_f64 * t119 * t38001 * t150 * t187 + 0.26020884564615598386e1_f64 * t32124 * t8306 * t36547 + 0.26341796731742046394e1_f64 * t8316 * t1620 - 0.17347256376410398924e1_f64 * t33198;
    t38270
}
