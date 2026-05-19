//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 887/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk887<F: Float>(t11936: F, t3470: F, t44045: F, t2660: F, t37180: F, t2684: F, t45369: F, t7585: F, t13589: F, t15362: F, t11823: F, t22256: F) -> (F, F, F, F, F, F) {
    let t45874 = F::cast_from(0.25025342966295298669e1_f64) * t11936 * t3470;
    let t45877 = F::cast_from(0.25561950635947166451e0_f64) * t44045;
    let t45882 = F::cast_from(0.10725146985555128001e1_f64) * t37180 * t2660;
    let t45885 = F::cast_from(0.14953741122029092374e3_f64) * t2684 * t7585 * t45369;
    let t45886 = t15362 * t13589;
    let t45887 = F::cast_from(0.29792074959875355558e-1_f64) * t45886;
    let t45888 = t11823 * t22256;
    (t45874, t45877, t45882, t45885, t45887, t45888)
}
