//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 932/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk932<F: Float>(t1330: F, t749: F, t512: F, t1320: F, t1331: F, t1340: F, t2516: F, t2496: F, t177: F, t762: F, t2626: F, t3827: F, t3856: F, t3859: F, t3862: F, t3865: F, t3867: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4029 = t1330 * t749;
    let t4030 = t512 * t4029;
    let t4031 = F::cast_from(2.0_f64) * t4030;
    let t4032 = t1320 * t1331;
    let t4033 = F::cast_from(8.0_f64) * t4032;
    let t4035 = F::cast_from(0.5848223622634646207e0_f64) * t1340 * t2516;
    let t4037 = F::cast_from(0.17315859105681463759e2_f64) * t1340 * t2496;
    let t4038 = t1330 * t177;
    let t4039 = t4038 * t762;
    let t4040 = F::cast_from(0.11696447245269292414e1_f64) * t4039;
    let t4042 = F::cast_from(0.11696447245269292414e1_f64) * t1340 * t2626;
    let t4043 = t3856 + t4031 - t4033 - t3867 - t4035 - t4037 - t4040 + t3859 + t3862 - t3865 - t3827 + t4042;
    (t4029, t4030, t4031, t4032, t4033, t4035, t4037, t4038, t4039, t4040, t4042, t4043)
}
