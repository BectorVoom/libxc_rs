//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1248/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1248<F: Float>(t7058: F, t93146: F, t11009: F, t1949: F, t1956: F, t1957: F, t233: F, t25391: F, t25392: F, t25416: F, t2723: F, t27353: F, t27357: F, t39588: F, t7070: F, t7073: F, t92935: F, t92937: F, t93099: F, t93104: F, t93112: F, t93116: F, t93118: F, t93124: F, t93126: F, t93130: F, t93138: F, t93142: F, t93143: F) -> F {
    let t93147 = t7058 * t93146;
    let t93149 = F::cast_from(0.19514881078765566037e-2_f64) * t92935 - F::cast_from(0.26020884564615598386e1_f64) * t7070 * t25416 * t92937 * t2723 - F::cast_from(0.4336814094102599731e0_f64) * t1956 * t1957 * t233 * t93099 - F::cast_from(0.26020884564615598386e1_f64) * t25391 * t25392 * t93104 - F::cast_from(0.26020884564615598386e1_f64) * t27353 * t27357 * t39588 - F::cast_from(0.72280234901709995519e-3_f64) * t93112 - F::cast_from(0.72280234901709995519e-3_f64) * t93116 + F::cast_from(0.10408353825846239354e2_f64) * t7070 * t93118 * t1949 * t11009 + F::cast_from(0.38554277296572111609e-1_f64) * t93124 + F::cast_from(0.26020884564615598386e1_f64) * t93126 * t7073 - F::cast_from(0.26020884564615598386e1_f64) * t25391 * t25392 * t93130 + t93138 - t93142 + F::cast_from(0.43368140941025997312e-1_f64) * t93143 + F::cast_from(0.21684070470512998656e-1_f64) * t93147;
    t93149
}
