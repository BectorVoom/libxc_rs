//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 536/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk536<F: Float>(t10392: F, t4781: F, t9371: F, t10268: F, t4820: F, t6824: F, t2478: F, t993: F, t6576: F, t2890: F, t6583: F, t2482: F) -> (F, F, F, F, F, F, F, F) {
    let t10394 = F::new(0.15337170381568299871e1) * t4781 * t10392;
    let t10395 = F::new(0.15976219147466979032e-1) * t9371;
    let t10396 = t4820 * t10268;
    let t10398 = F::new(0.79445533226334281487e-1) * t6824 * t10396;
    let t10399 = t993 * t2478;
    let t10400 = t6576 * t10399;
    let t10401 = F::new(0.19171462976960374838e0) * t10400;
    let t10402 = t2890 * t2478;
    let t10403 = t6583 * t10402;
    let t10404 = F::new(0.19171462976960374838e0) * t10403;
    let t10409 = t993 * t2482;
    (t10394, t10395, t10398, t10400, t10401, t10403, t10404, t10409)
}
