//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1015/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1015<F: Float>(t1275: F, t4101: F, t6124: F, t2129: F, t4032: F, t1253: F, t6079: F, t4075: F, t6082: F, t4083: F, t6078: F, t1264: F, t13583: F, t13588: F, t13705: F, t20266: F, t20270: F, t20273: F, t20278: F, t20282: F, t20329: F, t361: F, t4031: F, t4081: F, t4096: F, t4103: F, t4130: F, t6040: F, t6083: F, t6095: F, t6126: F) -> (F,) {
    let t20332 = t1275 * t4101;
    let t20333 = t6124 * t20332;
    let t20340 = t2129 * t4032;
    let t20347 = t6079 * t1253;
    let t20350 = t2129 * t4075;
    let t20353 = t6082 * t4032;
    let t20356 = t6078 * t4083;
    let t20357 = t20356 * t1253;
    let t20360 = 0.1038945353962551798e3 * t1264 * t20266 - 0.34631511798751726598e2 * t1264 * t20270 + 0.11696446794910408142e1 * t1264 * t20273 - 0.1025389702100779493e4 * t1264 * t20278 + 0.23392893589820816284e1 * t1264 * t20282 - 0.34631511798751726598e2 * t4096 * t6126 - 0.62182e-1 * t20329 * t361 - 0.35089340384731224426e1 * t1264 * t20333 + 0.11696446794910408142e1 * t6095 * t4103 - 0.17315755899375863299e2 * t6095 * t4130 + 6.0 * t4081 * t20340 - 4.0 * t13705 * t6040 + 0.32163648644302209644e2 * t13583 * t6083 - 4.0 * t4031 * t20347 - 2.0 * t4031 * t20350 - 0.96490945932906628932e2 * t13588 * t20353 + 0.32163648644302209644e2 * t4081 * t20357;
    (t20360,)
}
