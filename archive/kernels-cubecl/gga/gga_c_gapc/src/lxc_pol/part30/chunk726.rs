//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 726/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk726<F: Float>(t1404: F, t2880: F, t120: F, t118: F, t1803: F, t61: F, t1504: F, t1461: F, t4043: F, t1030: F, t3141: F, t5059: F) -> (F, F, F, F, F) {
    let t8585 = t2880 * t1404;
    let t8586 = t120 * t8585;
    let t8588 = t1803 * t118;
    let t8589 = t61 * t8588;
    let t8590 = t2880 * t1504;
    let t8591 = t8589 * t8590;
    let t8619 = t1461 * t4043;
    let t8620 = t1030 * t8619;
    let t8621 = t3141 * t5059;
    (t8586, t8591, t8619, t8620, t8621)
}
