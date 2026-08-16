//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1334/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1334(t1451: f64, t16082: f64, t1430: f64, t16060: f64, t16078: f64, t16069: f64, t16055: f64, t542: f64, t16065: f64, t111: f64, t120: f64, t12058: f64, t12061: f64, t12064: f64, t17155: f64, t17158: f64, t17161: f64, t17164: f64, t17167: f64, t17170: f64, t17174: f64, t17175: f64, t4865: f64, t4881: f64, t5820: f64) -> f64 {
    let t17177 = t1451 * t16082;
    let t17180 = t1430 * t16060;
    let t17183 = t1451 * t16078;
    let t17186 = t1430 * t16069;
    let t17189 = t542 * t16055;
    let t17192 = t1430 * t16065;
    let t17196 = -0.1585e-2_f64 * t111 * t17155 + 0.317e-2_f64 * t111 * t17158 + 0.634e-2_f64 * t4865 * t17161 - 0.52833333333333333333e-3_f64 * t111 * t17164 - 0.17611111111111111111e-3_f64 * t111 * t17167 + 0.21133333333333333334e-2_f64 * t4865 * t17170 + t17174 + 0.31368166666666666666e-4_f64 * t17175 - 0.10082625e-4_f64 * t120 * t17177 + 0.403305e-4_f64 * t120 * t17180 + 0.403305e-4_f64 * t4881 * t17183 - 0.672175e-5_f64 * t120 * t17186 + 0.22405833333333333333e-5_f64 * t120 * t17189 + 0.26887e-4_f64 * t4881 * t17192 + 0.10359077815592613752e-3_f64 * t5820 - t12058 + t12061 + t12064;
    t17196
}
