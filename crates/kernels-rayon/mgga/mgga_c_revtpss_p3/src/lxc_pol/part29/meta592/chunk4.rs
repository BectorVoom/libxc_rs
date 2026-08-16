//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1971/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1971(t102346: f64, t102361: f64, t102363: f64, t102364: f64, t102367: f64, t102372: f64, t13747: f64, t25921: f64, t26282: f64, t28850: f64, t28899: f64, t4131: f64, t4132: f64, t5728: f64, t7295: f64, t7296: f64, t7511: f64, t8085: f64, t96380: f64, t96382: f64, t96398: f64) -> f64 {
    let t102374 = -t102346 + 0.26341796731742046394e1_f64 * t7511 * t13747 - 0.65854491829355115987e0_f64 * t28899 * t4132 + 0.8673628188205199462e0_f64 * t25921 * t28850 + 0.34270468708064099208e-2_f64 * t96380 + 0.34270468708064099208e-2_f64 * t96382 + 0.8673628188205199462e0_f64 * t7295 * t7296 * t8085 * t4131 + t102361 + t102363 - 0.22849835011101738147e-2_f64 * t102364 + t102367 + 0.26341796731742046394e1_f64 * t26282 * t5728 - t102372 - 0.48186823267806663678e-3_f64 * t96398;
    t102374
}
