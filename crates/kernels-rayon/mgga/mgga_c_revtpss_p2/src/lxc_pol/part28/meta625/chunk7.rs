//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2230/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2230(t3151: f64, t7828: f64, t7150: f64, t99708: f64, t1089: f64, t16255: f64, t1647: f64, t1652: f64, t25476: f64, t25591: f64, t25605: f64, t25611: f64, t25634: f64, t25640: f64, t25662: f64, t27445: f64, t27550: f64, t27579: f64, t27606: f64, t27656: f64, t3076: f64, t3133: f64, t3304: f64, t4758: f64, t4947: f64, t4976: f64, t7140: f64, t7145: f64, t7153: f64, t7833: f64, t93464: f64, t93528: f64, t93884: f64, t93963: f64, t93983: f64, t988: f64, t99762: f64, t99877: f64) -> (f64, f64) {
    let t100490 = t7828 * t3151;
    let t100494 = t7150 * t99708;
    let t100513 = -0.65854491829355115987e0_f64 * t27550 * t3076 + 0.26341796731742046394e1_f64 * t25634 * t4947 + 0.26341796731742046394e1_f64 * t7140 * t16255 + 0.17347256376410398924e1_f64 * t93963 * t27656 + 0.17347256376410398924e1_f64 * t25611 * t99877 * t4976 + 0.17347256376410398924e1_f64 * t25611 * t99762 * t4976 + 0.8673628188205199462e0_f64 * t25605 * t7828 * t3133 * t1089 + 0.17347256376410398924e1_f64 * t93983 * t100490 * t3304 + 0.17347256376410398924e1_f64 * t100494 * t7153 - 0.4336814094102599731e0_f64 * t93464 * t7833 - 0.65854491829355115987e0_f64 * t93528 * t1652 + 0.26341796731742046394e1_f64 * t93884 * t4758 - 0.8673628188205199462e0_f64 * t25640 * t27606 - 0.17347256376410398924e1_f64 * t25476 * t27445 + 0.65854491829355115987e0_f64 * t1647 * t25662 + 0.34694512752820797848e1_f64 * t25591 * t7145 * t27579 * t988;
    (t100490, t100513)
}
