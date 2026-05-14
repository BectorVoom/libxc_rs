//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1112/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1112<F: Float>(t1928: F, t2941: F, t640: F, t11243: F, t8489: F, t11195: F, t24980: F, t152: F, t515: F, t2903: F, t623: F, t3945: F, t11189: F, t1845: F, t996: F, t11188: F, t1587: F, t3634: F) -> (F, F, F, F, F, F, F, F) {
    let t35562 = t2941 * t640 * t1928;
    let t35564 = t8489 * t11243;
    let t35566 = t24980 * t11195;
    let t35568 = t515 * t152;
    let t35570 = t2903 * t35568 * t623;
    let t35572 = t3945 * t11195;
    let t35575 = t996 * t1845 * t11189;
    let t35578 = t11188 * t3634 * t1587;
    (t35562, t35564, t35566, t35568, t35570, t35572, t35575, t35578)
}
