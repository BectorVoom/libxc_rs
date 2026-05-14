//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1008/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1008<F: Float>(t1882: F, t24528: F, t1424: F, t9570: F, t24479: F, t96925: F, t1433: F, t1771: F, t6121: F, t24444: F, t24543: F, t24560: F, t2: F, t24395: F, t24552: F, t1434: F, t1435: F, t9555: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t96968 = t1882 * t24528;
    let t96970 = t1424 * t9570;
    let t96975 = t96925 * t24479;
    let t96982 = t1433 * t1771;
    let t96983 = t96982 * t6121;
    let t96985 = t24543 * t24444;
    let t97003 = t24543 * t24560;
    let t97005 = t2 * t24395;
    let t97022 = t24543 * t24552;
    let t97029 = t1434 * t9555 * t1435;
    (t96968, t96970, t96975, t96982, t96983, t96985, t97003, t97005, t97022, t97029)
}
