//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2230/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2230<F: Float>(t3151: F, t7828: F, t7150: F, t99708: F, t1089: F, t16255: F, t1647: F, t1652: F, t25476: F, t25591: F, t25605: F, t25611: F, t25634: F, t25640: F, t25662: F, t27445: F, t27550: F, t27579: F, t27606: F, t27656: F, t3076: F, t3133: F, t3304: F, t4758: F, t4947: F, t4976: F, t7140: F, t7145: F, t7153: F, t7833: F, t93464: F, t93528: F, t93884: F, t93963: F, t93983: F, t988: F, t99762: F, t99877: F) -> (F, F) {
    let t100490 = t7828 * t3151;
    let t100494 = t7150 * t99708;
    let t100513 = -F::cast_from(0.65854491829355115987e0_f64) * t27550 * t3076 + F::cast_from(0.26341796731742046394e1_f64) * t25634 * t4947 + F::cast_from(0.26341796731742046394e1_f64) * t7140 * t16255 + F::cast_from(0.17347256376410398924e1_f64) * t93963 * t27656 + F::cast_from(0.17347256376410398924e1_f64) * t25611 * t99877 * t4976 + F::cast_from(0.17347256376410398924e1_f64) * t25611 * t99762 * t4976 + F::cast_from(0.8673628188205199462e0_f64) * t25605 * t7828 * t3133 * t1089 + F::cast_from(0.17347256376410398924e1_f64) * t93983 * t100490 * t3304 + F::cast_from(0.17347256376410398924e1_f64) * t100494 * t7153 - F::cast_from(0.4336814094102599731e0_f64) * t93464 * t7833 - F::cast_from(0.65854491829355115987e0_f64) * t93528 * t1652 + F::cast_from(0.26341796731742046394e1_f64) * t93884 * t4758 - F::cast_from(0.8673628188205199462e0_f64) * t25640 * t27606 - F::cast_from(0.17347256376410398924e1_f64) * t25476 * t27445 + F::cast_from(0.65854491829355115987e0_f64) * t1647 * t25662 + F::cast_from(0.34694512752820797848e1_f64) * t25591 * t7145 * t27579 * t988;
    (t100490, t100513)
}
