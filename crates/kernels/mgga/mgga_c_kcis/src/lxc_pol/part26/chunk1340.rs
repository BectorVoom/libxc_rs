//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1340/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1340<F: Float>(t1528: F, t7386: F, t570: F, t7052: F, t7953: F, t16752: F, t2055: F, t2043: F, t5998: F, t1468: F, t22652: F, t22324: F, t7948: F) -> (F, F, F, F, F, F) {
    let t102939 = t1528 * t7386;
    let t102941 = t570 * t7052;
    let t102942 = t102941 * t7953;
    let t102944 = t16752 * t2055;
    let t102946 = t5998 * t2043;
    let t102948 = t1468 * t22652;
    let t102950 = t7948 * t22324;
    (t102939, t102942, t102944, t102946, t102948, t102950)
}
