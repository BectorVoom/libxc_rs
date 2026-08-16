//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1010/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1010<F: Float>(t2297: F, t8177: F, t1197: F, t2258: F, t3038: F, t6317: F, t3074: F, t6137: F, t2279: F, t2296: F, t2318: F, t3121: F, t3140: F, t6266: F, t6282: F, t6288: F, t6300: F, t6323: F, t8150: F, t8154: F, t8161: F, t8164: F, t8167: F, t8171: F, t8174: F) -> (F, F, F, F, F) {
    let t8178 = t8177 * t2297;
    let t8181 = t1197 * t2258;
    let t8185 = F::cast_from(4.0_f64) * t6317 * t3038;
    let t8187 = F::cast_from(0.32163958997385070134e2_f64) * t6137 * t3074;
    let t8188 = F::cast_from(0.35089341735807877242e1_f64) * t2318 * t8150 + F::cast_from(0.2069040516770936012e4_f64) * t6288 * t8154 - F::cast_from(0.23392894490538584828e1_f64) * t6266 * t3121 + F::cast_from(0.34631718211362927518e2_f64) * t6300 * t3140 - F::cast_from(0.23392894490538584828e1_f64) * t2296 * t8161 - F::cast_from(0.11696447245269292414e1_f64) * t2296 * t8164 - F::cast_from(0.10389515463408878255e3_f64) * t6323 * t8167 + F::cast_from(0.34631718211362927518e2_f64) * t2318 * t8171 + F::cast_from(0.17315859105681463759e2_f64) * t2318 * t8174 + F::cast_from(0.10254018858216406658e4_f64) * t6282 * t8178 + F::cast_from(6.0_f64) * t2279 * t8181 + t8185 - t8187;
    (t8178, t8181, t8185, t8187, t8188)
}
