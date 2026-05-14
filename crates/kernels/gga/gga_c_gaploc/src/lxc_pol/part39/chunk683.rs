//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 683/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk683<F: Float>(t12957: F, t9263: F, t2890: F, t3177: F, t9267: F, t123: F, t3338: F, t883: F, t912: F, t587: F, t3129: F, t900: F, t10615: F, t9448: F, t986: F, t9438: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12958 = t9263 * t12957;
    let t12959 = 0.76685851907841499353e0 * t12958;
    let t12960 = t2890 * t3177;
    let t12961 = t9267 * t12960;
    let t12962 = 0.19171462976960374838e1 * t12961;
    let t12963 = t3338 * t123;
    let t12964 = t12963 * t883;
    let t12965 = t912 * t12964;
    let t12966 = t587 * t12965;
    let t12968 = t900 * t3129;
    let t12969 = t10615 * t12968;
    let t12970 = 0.89376224879626066675e-1 * t12969;
    let t12986 = t9448 * t986;
    let t12987 = t9438 * t12986;
    (t12959, t12960, t12962, t12963, t12964, t12965, t12966, t12968, t12970, t12986, t12987)
}
