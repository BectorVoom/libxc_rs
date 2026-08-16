//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2103/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2103(t1398: f64, t1445: f64, t2030: f64, t25909: f64, t26084: f64, t27868: f64, t27960: f64, t27980: f64, t48025: f64, t543: f64, t5658: f64, t5728: f64, t7274: f64, t7295: f64, t7301: f64, t7304: f64, t7930: f64, t94820: f64, t94842: f64, t94844: f64, t94851: f64, t98029: f64, t98043: f64, t98050: f64, t98053: f64, t98056: f64) -> f64 {
    let t98061 = -0.48186823267806663678e-3_f64 * t94820 - 0.8673628188205199462e0_f64 * t27868 * t27980 * t48025 + 0.19274729307122665472e-1_f64 * t98029 + 0.8673628188205199462e0_f64 * t7295 * t7301 * t27960 * t1398 * t543 - 0.4336814094102599731e0_f64 * t25909 * t7930 + 0.19274729307122665471e-1_f64 * t94842 - 0.72280234901709995518e-2_f64 * t94844 + t98043 + 0.8673628188205199462e0_f64 * t7295 * t7301 * t7274 * t5658 * t543 + 0.48186823267806663678e-3_f64 * t94851 + 0.8673628188205199462e0_f64 * t98050 * t7304 - 0.4336814094102599731e0_f64 * t98053 * t2030 - 0.13170898365871023197e1_f64 * t98056 * t1445 + 0.26341796731742046394e1_f64 * t26084 * t5728;
    t98061
}
