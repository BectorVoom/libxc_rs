//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 714/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk714<F: Float>(t647: F, t8846: F, t2998: F, t2996: F, t1030: F, t8838: F, t3006: F, t5298: F, t8841: F, t1896: F, t3034: F, t1026: F, t1804: F, t1808: F, t1850: F, t3039: F) -> (F, F, F, F, F, F, F) {
    let t8847 = t647 * t8846;
    let t8848 = t2998 * t8847;
    let t8849 = t2996 * t8848;
    let t8851 = t1030 * t8838;
    let t8852 = t5298 * t3006;
    let t8853 = t8841 * t8852;
    let t8854 = t8851 * t8853;
    let t8856 = t3034 * t1896;
    let t8858 = t1804 * t1026;
    let t8859 = t8858 * t1808;
    let t8861 = t3039 * t1850;
    (t8848, t8849, t8853, t8854, t8856, t8859, t8861)
}
