//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 630/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk630<F: Float>(t12958: F, t2890: F, t3177: F, t9267: F, t123: F, t3338: F, t883: F, t912: F, t587: F, t3129: F, t900: F, t10615: F, t12792: F, t189: F, t188: F, t600: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
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
    let t12971 = t189 * t12792;
    let t12972 = t188 * t12971;
    let t12975 = t600 * t12792;
    (t12959, t12960, t12962, t12963, t12964, t12965, t12966, t12968, t12970, t12971, t12972, t12975)
}
