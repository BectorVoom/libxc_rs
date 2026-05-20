//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 908/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk908<F: Float>(t4029: F, t512: F, t1320: F, t1331: F, t1340: F, t2516: F, t2496: F, t1330: F, t177: F) -> (F, F, F, F, F) {
    let t4030 = t512 * t4029;
    let t4032 = t1320 * t1331;
    let t4035 = F::cast_from(0.5848223622634646207e0_f64) * t1340 * t2516;
    let t4037 = F::cast_from(0.17315859105681463759e2_f64) * t1340 * t2496;
    let t4038 = t1330 * t177;
    (t4030, t4032, t4035, t4037, t4038)
}
