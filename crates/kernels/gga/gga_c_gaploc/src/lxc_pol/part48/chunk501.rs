//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 501/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk501<F: Float>(t1580: F, t3399: F, t10140: F, t1445: F, t597: F, t10144: F, t10123: F, t10241: F, t4130: F, t590: F, t4781: F, t9371: F, t10268: F, t4820: F, t6824: F, t2478: F, t993: F) -> (F, F, F, F, F, F, F, F) {
    let t10381 = 0.11502877786176224903e2 * t1580 * t3399;
    let t10382 = t1445 * t10140;
    let t10384 = 0.11502877786176224903e2 * t597 * t10382;
    let t10385 = t1445 * t10144;
    let t10387 = 0.11502877786176224903e2 * t597 * t10385;
    let t10388 = t1445 * t10123;
    let t10392 = t4130 * t10241 * t590;
    let t10394 = 0.15337170381568299871e1 * t4781 * t10392;
    let t10395 = 0.15976219147466979032e-1 * t9371;
    let t10396 = t4820 * t10268;
    let t10398 = 0.79445533226334281487e-1 * t6824 * t10396;
    let t10399 = t993 * t2478;
    (t10381, t10384, t10387, t10388, t10394, t10395, t10398, t10399)
}
