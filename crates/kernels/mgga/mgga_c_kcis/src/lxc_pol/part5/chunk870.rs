//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 870/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk870<F: Float>(t1451: F, t6912: F, t1430: F, t6944: F, t542: F, t6937: F, t1437: F, t1330: F, t104: F, t111: F, t120: F, t1404: F, t1445: F, t4093: F, t6281: F, t6284: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7158 = t1451 * t6912;
    let t7161 = t1430 * t6944;
    let t7164 = t542 * t6937;
    let t7167 = t1437 * t6944;
    let t7170 = t1330 * t6937;
    let t7173 = t1451 * t6944;
    let t7176 = t1430 * t6937;
    let t7183 = t1430 * t6912;
    let t7186 = t1437 * t6912;
    let t7189 = F::cast_from(0.15538616723388920628e-3_f64) * t4093 * t6281 - F::cast_from(0.10082625e-4_f64) * t120 * t7158 - F::cast_from(0.3513e-2_f64) * t104 * t7161 + F::cast_from(0.1171e-2_f64) * t104 * t7164 + F::cast_from(0.7925e-3_f64) * t111 * t7167 - F::cast_from(0.52833333333333333333e-3_f64) * t111 * t7170 + F::cast_from(0.50413125e-5_f64) * t120 * t7173 - F::cast_from(0.672175e-5_f64) * t120 * t7176 + F::cast_from(0.11955719325063177623e-1_f64) * t1404 * t6284 - F::cast_from(0.5179538907796306876e-4_f64) * t1445 * t6284 + F::cast_from(0.7026e-2_f64) * t104 * t7183 - F::cast_from(0.1585e-2_f64) * t111 * t7186;
    (t7158, t7161, t7164, t7167, t7170, t7173, t7176, t7183, t7186, t7189)
}
