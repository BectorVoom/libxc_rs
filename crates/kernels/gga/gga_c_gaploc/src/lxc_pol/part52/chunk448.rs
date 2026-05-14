//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 448/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk448<F: Float>(t6508: F, t7892: F, t447: F, t986: F, t2366: F, t2754: F, t555: F, t1570: F, t1352: F, t987: F, t203: F, t599: F, t158: F, t2796: F, t501: F, t1381: F, t997: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7893 = t6508 * t7892;
    let t7905 = t986 * t447;
    let t7906 = t2366 * t7905;
    let t7930 = t555 * t2754;
    let t7937 = t1570 * t986;
    let t7974 = t987 * t1352;
    let t7980 = t203 * t2754;
    let t7995 = t599 * t2754;
    let t8025 = t158 * t2754;
    let t8042 = t2796 * t501;
    let t8045 = t997 * t1381;
    (t7893, t7905, t7906, t7930, t7937, t7974, t7980, t7995, t8025, t8042, t8045)
}
