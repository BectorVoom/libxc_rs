//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 503/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk503<F: Float>(t531: F, t549: F, t240: F, t72: F, t1386: F, t2482: F, t27: F, t136: F, t1389: F, t1317: F, t1333: F, t1340: F, t2516: F, t2496: F, t2626: F, t1412: F, t73: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4010 = 1.0 / t549 / t531;
    let t4011 = t240 * t4010;
    let t4012 = t4011 * t72;
    let t4018 = t2482 * t1386 * t27;
    let t4019 = t1389 * t136;
    let t4027 = 8.0 * t1317 * t1333;
    let t4035 = 0.5848223622634646207e0 * t1340 * t2516;
    let t4037 = 0.17315859105681463759e2 * t1340 * t2496;
    let t4042 = 0.11696447245269292414e1 * t1340 * t2626;
    let t4049 = t73 * t1412;
    (t4010, t4011, t4012, t4018, t4019, t4027, t4035, t4037, t4042, t4049)
}
