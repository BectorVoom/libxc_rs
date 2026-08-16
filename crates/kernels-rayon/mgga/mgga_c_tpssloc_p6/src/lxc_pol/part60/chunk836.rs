//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 836/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk836(t29580: f64, t29610: f64, t29636: f64, t29662: f64, t466: f64, t1238: f64, t1761: f64, t27406: f64, t27792: f64, t29532: f64, t29536: f64, t29546: f64, t29551: f64, t29554: f64, t29557: f64, t498: f64, t5055: f64, t6244: f64, t7283: f64, t7351: f64, t8003: f64, t8061: f64) -> (f64, f64) {
    let t29664 = t29580 + t29610 + t29636 + t29662;
    let t29665 = t466 * t29664;
    let t29667 = 4.0_f64 * t1238 * t29532 + 2.0_f64 * t1238 * t29536 - 2.0_f64 * t27792 * t1761 + 4.0_f64 * t5055 * t8061 + 0.14621636149762012769e-1_f64 * t27406 * t8003 - 0.82246703342411321825e-2_f64 * t7283 * t29546 + 2.0_f64 * t7351 * t6244 - 0.82246703342411321825e-2_f64 * t7283 * t29551 - 0.16449340668482264365e-1_f64 * t7283 * t29554 + 0.16449340668482264365e-1_f64 * t7283 * t29557 + t29665 * t498;
    (t29664, t29667)
}
