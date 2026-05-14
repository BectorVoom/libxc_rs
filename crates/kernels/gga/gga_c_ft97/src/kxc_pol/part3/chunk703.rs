//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 703/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk703<F: Float>(t11922: F, t11930: F, t11931: F, t11932: F, t15942: F, t15945: F, t15948: F, t15953: F, t15957: F, t15961: F, t8443: F, t15902: F, t1787: F, t1775: F, t4515: F, t15913: F, t8291: F) -> (F, F, F, F) {
    let t16365 = 2.0 / 9.0 * t15942 + 4.0 / 3.0 * t15945 - 10.0 / 27.0 * t15948 - t11922 - t11930 - t11931 + t11932 - 4.0 / 3.0 * t15953 + 4.0 / 9.0 * t15957 - 4.0 / 3.0 * t15961 - t8443;
    let t16370 = t1787 * t15902;
    let t16373 = t1775 * t4515;
    let t16375 = t8291 * t15913;
    (t16365, t16370, t16373, t16375)
}
