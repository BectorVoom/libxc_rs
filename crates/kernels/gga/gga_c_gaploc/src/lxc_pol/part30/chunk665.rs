//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 665/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk665<F: Float>(t5679: F, t789: F, t1: F, t5501: F, t787: F, t4371: F, t734: F, t2066: F, t796: F, t4752: F, t702: F, t1645: F, t1836: F) -> (F, F, F, F, F, F) {
    let t5680 = t5679 * t789;
    let t5687 = t5501 * t1;
    let t5688 = t787 * t5687;
    let t5694 = t4371 * t734;
    let t5703 = t2066 * t796;
    let t5715 = t4752 * t702;
    let t5724 = t1645 * t1836;
    (t5680, t5688, t5694, t5703, t5715, t5724)
}
