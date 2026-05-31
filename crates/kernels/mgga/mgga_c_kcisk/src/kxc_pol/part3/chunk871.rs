//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 871/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk871<F: Float>(t12998: F, t12974: F, t12959: F, t12962: F, t12965: F, t12967: F, t12971: F, t12985: F, t12989: F, t12993: F, t12995: F, t13002: F, t13005: F, t13010: F) -> F {
    let t13091 = F::cast_from(0.36793333333333333333e0_f64) * t12998;
    let t13092 = F::cast_from(0.93932222222222222223e0_f64) * t12974;
    let t13098 = -F::cast_from(0.181155e1_f64) * t12959 + F::cast_from(0.16557e0_f64) * t12962 - F::cast_from(0.49671e0_f64) * t12965 - F::cast_from(0.33114e0_f64) * t12967 - F::cast_from(0.412621875e-1_f64) * t12971 + F::cast_from(0.258925e1_f64) * t12993 + F::cast_from(0.16504875e0_f64) * t12995 - t13091 - t13092 - F::cast_from(0.82785e-1_f64) * t13002 + F::cast_from(0.49671e0_f64) * t13005 + F::cast_from(0.19419375e1_f64) * t13010 - F::cast_from(0.60384999999999999999e0_f64) * t12985 + F::cast_from(0.181155e1_f64) * t12989;
    t13098
}
