//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1116/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1116<F: Float>(t11192: F, t2906: F, t1504: F, t1803: F, t3634: F, t996: F, t1404: F, t997: F, t1510: F, t14541: F, t1458: F, t1649: F, t474: F, t11199: F, t8419: F, t11189: F, t8524: F) -> (F, F, F, F, F, F, F) {
    let t35580 = t2906 * t11192;
    let t35584 = t996 * t1803 * t3634 * t1504;
    let t35588 = t997 * t3634 * t1404;
    let t35591 = t997 * t3634 * t1510;
    let t35595 = t14541 * t1458 * t474 * t1649;
    let t35597 = t8419 * t11199;
    let t35599 = t8524 * t11189;
    (t35580, t35584, t35588, t35591, t35595, t35597, t35599)
}
