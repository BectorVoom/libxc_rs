//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1010/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1010(t2297: f64, t8177: f64, t1197: f64, t2258: f64, t3038: f64, t6317: f64, t3074: f64, t6137: f64, t2279: f64, t2296: f64, t2318: f64, t3121: f64, t3140: f64, t6266: f64, t6282: f64, t6288: f64, t6300: f64, t6323: f64, t8150: f64, t8154: f64, t8161: f64, t8164: f64, t8167: f64, t8171: f64, t8174: f64) -> (f64, f64, f64, f64, f64) {
    let t8178 = t8177 * t2297;
    let t8181 = t1197 * t2258;
    let t8185 = 4.0_f64 * t6317 * t3038;
    let t8187 = 0.32163958997385070134e2_f64 * t6137 * t3074;
    let t8188 = 0.35089341735807877242e1_f64 * t2318 * t8150 + 0.2069040516770936012e4_f64 * t6288 * t8154 - 0.23392894490538584828e1_f64 * t6266 * t3121 + 0.34631718211362927518e2_f64 * t6300 * t3140 - 0.23392894490538584828e1_f64 * t2296 * t8161 - 0.11696447245269292414e1_f64 * t2296 * t8164 - 0.10389515463408878255e3_f64 * t6323 * t8167 + 0.34631718211362927518e2_f64 * t2318 * t8171 + 0.17315859105681463759e2_f64 * t2318 * t8174 + 0.10254018858216406658e4_f64 * t6282 * t8178 + 6.0_f64 * t2279 * t8181 + t8185 - t8187;
    (t8178, t8181, t8185, t8187, t8188)
}
