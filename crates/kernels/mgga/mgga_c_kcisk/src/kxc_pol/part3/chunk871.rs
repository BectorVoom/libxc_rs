//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 871/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk871<F: Float>(t12998: F, t12974: F, t12959: F, t12962: F, t12965: F, t12967: F, t12971: F, t12985: F, t12989: F, t12993: F, t12995: F, t13002: F, t13005: F, t13010: F) -> F {
    let t13091 = F::new(0.36793333333333333333e0) * t12998;
    let t13092 = F::new(0.93932222222222222223e0) * t12974;
    let t13098 = -F::new(0.181155e1) * t12959 + F::new(0.16557e0) * t12962 - F::new(0.49671e0) * t12965 - F::new(0.33114e0) * t12967 - F::new(0.412621875e-1) * t12971 + F::new(0.258925e1) * t12993 + F::new(0.16504875e0) * t12995 - t13091 - t13092 - F::new(0.82785e-1) * t13002 + F::new(0.49671e0) * t13005 + F::new(0.19419375e1) * t13010 - F::new(0.60384999999999999999e0) * t12985 + F::new(0.181155e1) * t12989;
    t13098
}
