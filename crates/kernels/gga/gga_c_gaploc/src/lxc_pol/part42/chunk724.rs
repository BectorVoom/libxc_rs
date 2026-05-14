//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 724/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk724<F: Float>(t13265: F, t2312: F, t1063: F, t11264: F, t6755: F, t2268: F, t35045: F, t7937: F, t42827: F, t11232: F, t894: F, t2440: F, t3531: F, t6949: F, t13277: F, t6305: F) -> (F, F, F, F, F, F, F, F) {
    let t44543 = t2312 * t13265;
    let t44544 = 0.35568758294595186999e-2 * t44543;
    let t44549 = 0.34146007962811379518e0 * t1063 * t11264 * t6755;
    let t44552 = 0.68292015925622759036e0 * t2268 * t7937 * t35045;
    let t44553 = 0.47425011059460249332e-2 * t42827;
    let t44556 = 0.28455006635676149599e-1 * t2268 * t894 * t11232;
    let t44559 = 0.28455006635676149599e-1 * t2268 * t2440 * t3531;
    let t44572 = 0.34146007962811379518e0 * t2268 * t11264 * t6949;
    let t44574 = 0.17073003981405689759e0 * t6305 * t13277;
    (t44544, t44549, t44552, t44553, t44556, t44559, t44572, t44574)
}
