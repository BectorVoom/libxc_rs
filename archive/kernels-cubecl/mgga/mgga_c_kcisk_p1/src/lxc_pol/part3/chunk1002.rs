//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1002/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1002<F: Float>(t12998: F, t12974: F, t12959: F, t12962: F, t12965: F, t12967: F, t12971: F, t12985: F, t12989: F, t12993: F, t12995: F, t13002: F, t13005: F, t13010: F) -> F {
    let t14784 = F::cast_from(0.46308888888888888888e0_f64) * t12998;
    let t14785 = F::cast_from(0.16068111111111111111e1_f64) * t12974;
    let t14791 = -F::cast_from(0.309885e1_f64) * t12959 + F::cast_from(0.20839e0_f64) * t12962 - F::cast_from(0.62517e0_f64) * t12965 - F::cast_from(0.41678000000000000001e0_f64) * t12967 - F::cast_from(0.157790625e0_f64) * t12971 + F::cast_from(0.3529725e1_f64) * t12993 + F::cast_from(0.6311625e0_f64) * t12995 - t14784 - t14785 - F::cast_from(0.104195e0_f64) * t13002 + F::cast_from(0.62517e0_f64) * t13005 + F::cast_from(0.264729375e1_f64) * t13010 - F::cast_from(0.103295e1_f64) * t12985 + F::cast_from(0.309885e1_f64) * t12989;
    t14791
}
