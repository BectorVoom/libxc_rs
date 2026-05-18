//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 797/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk797<F: Float>(t15902: F, t1787: F, t1775: F, t4515: F, t15913: F, t8291: F, t15927: F, t15768: F, t3134: F, t15763: F, t3127: F, t15936: F) -> (F, F, F, F, F, F, F) {
    let t16370 = t1787 * t15902;
    let t16373 = t1775 * t4515;
    let t16375 = t8291 * t15913;
    let t16378 = t1787 * t15927;
    let t16381 = t3134 * t15768;
    let t16384 = t3127 * t15763;
    let t16387 = t1787 * t15936;
    (t16370, t16373, t16375, t16378, t16381, t16384, t16387)
}
