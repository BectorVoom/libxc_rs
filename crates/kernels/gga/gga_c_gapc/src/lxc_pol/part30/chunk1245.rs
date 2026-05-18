//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1245/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1245<F: Float>(t11189: F, t1845: F, t996: F, t11188: F, t1587: F, t3634: F, t11192: F, t2906: F, t1504: F, t1803: F, t1404: F, t997: F) -> (F, F, F, F, F) {
    let t35575 = t996 * t1845 * t11189;
    let t35578 = t11188 * t3634 * t1587;
    let t35580 = t2906 * t11192;
    let t35584 = t996 * t1803 * t3634 * t1504;
    let t35588 = t997 * t3634 * t1404;
    (t35575, t35578, t35580, t35584, t35588)
}
