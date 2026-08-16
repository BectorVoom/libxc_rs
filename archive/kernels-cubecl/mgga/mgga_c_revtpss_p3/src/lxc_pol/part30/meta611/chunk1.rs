//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2091/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2091<F: Float>(t1398: F, t1445: F, t2030: F, t25909: F, t26084: F, t27868: F, t27960: F, t27980: F, t48025: F, t543: F, t5658: F, t5728: F, t7274: F, t7295: F, t7301: F, t7304: F, t7930: F, t94820: F, t94842: F, t94844: F, t94851: F, t98029: F, t98043: F, t98050: F, t98053: F, t98056: F) -> F {
    let t98061 = -F::cast_from(0.48186823267806663678e-3_f64) * t94820 - F::cast_from(0.8673628188205199462e0_f64) * t27868 * t27980 * t48025 + F::cast_from(0.19274729307122665472e-1_f64) * t98029 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7301 * t27960 * t1398 * t543 - F::cast_from(0.4336814094102599731e0_f64) * t25909 * t7930 + F::cast_from(0.19274729307122665471e-1_f64) * t94842 - F::cast_from(0.72280234901709995518e-2_f64) * t94844 + t98043 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7301 * t7274 * t5658 * t543 + F::cast_from(0.48186823267806663678e-3_f64) * t94851 + F::cast_from(0.8673628188205199462e0_f64) * t98050 * t7304 - F::cast_from(0.4336814094102599731e0_f64) * t98053 * t2030 - F::cast_from(0.13170898365871023197e1_f64) * t98056 * t1445 + F::cast_from(0.26341796731742046394e1_f64) * t26084 * t5728;
    t98061
}
