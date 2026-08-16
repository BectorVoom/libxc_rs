//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2232/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2232<F: Float>(t3143: F, t7810: F, t1977: F, t994: F, t11627: F, t1983: F, t99682: F, t1089: F, t12132: F, t15886: F, t16344: F, t1652: F, t16554: F, t16592: F, t16605: F, t1978: F, t25461: F, t25476: F, t25484: F, t25487: F, t25651: F, t25671: F, t27419: F, t27557: F, t27604: F, t27635: F, t27642: F, t27669: F, t3133: F, t3151: F, t3304: F, t3318: F, t4743: F, t4983: F, t7137: F, t7140: F, t7167: F, t7837: F, t93459: F, t99685: F, t99735: F) -> F {
    let t100567 = t3143 * t7810;
    let t100586 = t994 * t1977;
    let t100596 = t1983 * t99682 * t11627;
    let t100606 = -F::cast_from(0.4336814094102599731e0_f64) * t25487 * t7837 - F::cast_from(0.4336814094102599731e0_f64) * t7167 * t27604 * t3133 * t1089 - F::cast_from(0.8673628188205199462e0_f64) * t25671 * t100567 * t3151 * t3304 + F::cast_from(0.4336814094102599731e0_f64) * t25671 * t27604 * t3151 * t3318 + F::cast_from(0.8673628188205199462e0_f64) * t27419 * t25484 - F::cast_from(0.13170898365871023197e1_f64) * t25651 * t16344 + F::cast_from(0.17347256376410398924e1_f64) * t25461 * t27557 + F::cast_from(0.65854491829355115987e0_f64) * t15886 * t1978 + F::cast_from(0.13170898365871023197e1_f64) * t4743 * t7137 - F::cast_from(0.26341796731742046394e1_f64) * t100586 * t16605 - F::cast_from(0.17347256376410398924e1_f64) * t27669 * t99735 * t4983 - F::cast_from(0.8673628188205199462e0_f64) * t27669 * t27642 * t12132 - F::cast_from(0.26020884564615598386e1_f64) * t100596 * t99685 * t16554 - F::cast_from(0.65854491829355115987e0_f64) * t7140 * t16592 - F::cast_from(0.13170898365871023197e1_f64) * t93459 * t1652 + F::cast_from(0.34694512752820797848e1_f64) * t25476 * t27635;
    t100606
}
