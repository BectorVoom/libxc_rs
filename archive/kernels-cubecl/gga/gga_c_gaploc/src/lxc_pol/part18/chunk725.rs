//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 725/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk725<F: Float>(t1445: F, t6784: F, t6424: F, t2389: F, t2410: F, t1457: F, t6443: F, t2335: F, t4673: F, t2398: F, t4614: F, t2378: F) -> (F, F, F, F, F, F, F) {
    let t6785 = t1445 * t6784;
    let t6790 = t1445 * t6424;
    let t6793 = t2410 * t2389;
    let t6795 = t1457 * t6443;
    let t6798 = t4673 * t2335;
    let t6801 = t4614 * t2398;
    let t6804 = t4614 * t2378;
    (t6785, t6790, t6793, t6795, t6798, t6801, t6804)
}
