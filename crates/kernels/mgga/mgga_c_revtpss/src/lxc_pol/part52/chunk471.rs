//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 471/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk471<F: Float>(t1330: F, t749: F, t512: F, t1320: F, t1331: F, t1340: F, t2516: F, t2496: F, t177: F, t762: F, t2626: F, t1389: F, t1408: F, t2736: F, t1419: F, t213: F) -> (F, F, F, F, F, F, F, F) {
    let t4029 = t1330 * t749;
    let t4030 = t512 * t4029;
    let t4032 = t1320 * t1331;
    let t4035 = 0.5848223622634646207e0 * t1340 * t2516;
    let t4037 = 0.17315859105681463759e2 * t1340 * t2496;
    let t4038 = t1330 * t177;
    let t4039 = t4038 * t762;
    let t4042 = 0.11696447245269292414e1 * t1340 * t2626;
    let t4062 = t1408 * t1389;
    let t4064 = 0.25410001404642664112e-5 * t2736 * t4062;
    let t4071 = t213 * t1419;
    (t4030, t4032, t4035, t4037, t4039, t4042, t4064, t4071)
}
