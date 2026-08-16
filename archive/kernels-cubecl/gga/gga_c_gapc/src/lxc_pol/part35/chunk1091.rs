//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1091/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1091<F: Float>(t1736: F, t653: F, t2660: F, t327: F, t33546: F, t11508: F, t2664: F, t3363: F, t11513: F, t7294: F, t11325: F, t3789: F) -> (F, F, F, F, F, F) {
    let t33549 = t653 * t1736;
    let t33552 = t2660 * t33549 * t327 * t33546;
    let t33555 = t3363 * t11508 * t2664;
    let t33558 = t7294 * t11513 * t2664;
    let t33560 = t3363 * t11325;
    let t33561 = t33560 * t3789;
    (t33549, t33552, t33555, t33558, t33560, t33561)
}
