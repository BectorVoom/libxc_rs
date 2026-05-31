//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 796/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk796<F: Float>(t15904: F, t15907: F, t15910: F, t15915: F, t15919: F, t15922: F, t15925: F, t15929: F, t15934: F, t15938: F, t16346: F, t11922: F, t11930: F, t11931: F, t11932: F, t15942: F, t15945: F, t15948: F, t15953: F, t15957: F, t15961: F, t8443: F) -> (F, F) {
    let t16357 = -t16346 + t15904 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t15907 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t15910 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t15915 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t15919 - F::cast_from(2.0_f64) * t15922 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t15925 + t15929 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t15934 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t15938;
    let t16365 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t15942 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t15945 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t15948 - t11922 - t11930 - t11931 + t11932 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t15953 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t15957 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t15961 - t8443;
    (t16357, t16365)
}
