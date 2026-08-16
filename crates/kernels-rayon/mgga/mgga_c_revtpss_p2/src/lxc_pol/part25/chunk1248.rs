//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1248/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1248(t7058: f64, t93146: f64, t11009: f64, t1949: f64, t1956: f64, t1957: f64, t233: f64, t25391: f64, t25392: f64, t25416: f64, t2723: f64, t27353: f64, t27357: f64, t39588: f64, t7070: f64, t7073: f64, t92935: f64, t92937: f64, t93099: f64, t93104: f64, t93112: f64, t93116: f64, t93118: f64, t93124: f64, t93126: f64, t93130: f64, t93138: f64, t93142: f64, t93143: f64) -> f64 {
    let t93147 = t7058 * t93146;
    let t93149 = 0.19514881078765566037e-2_f64 * t92935 - 0.26020884564615598386e1_f64 * t7070 * t25416 * t92937 * t2723 - 0.4336814094102599731e0_f64 * t1956 * t1957 * t233 * t93099 - 0.26020884564615598386e1_f64 * t25391 * t25392 * t93104 - 0.26020884564615598386e1_f64 * t27353 * t27357 * t39588 - 0.72280234901709995519e-3_f64 * t93112 - 0.72280234901709995519e-3_f64 * t93116 + 0.10408353825846239354e2_f64 * t7070 * t93118 * t1949 * t11009 + 0.38554277296572111609e-1_f64 * t93124 + 0.26020884564615598386e1_f64 * t93126 * t7073 - 0.26020884564615598386e1_f64 * t25391 * t25392 * t93130 + t93138 - t93142 + 0.43368140941025997312e-1_f64 * t93143 + 0.21684070470512998656e-1_f64 * t93147;
    t93149
}
