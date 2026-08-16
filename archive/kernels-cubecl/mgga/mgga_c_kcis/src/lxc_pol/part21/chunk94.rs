//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 94/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk94<F: Float>(t242: F, t245: F, t248: F, t255: F) -> (F, F, F) {
    let t270 = F::cast_from(0.51785e1_f64) * t245 + F::cast_from(0.905775e0_f64) * t242 + F::cast_from(0.1100325e0_f64) * t248 + F::cast_from(0.1241775e0_f64) * t255;
    let t273 = F::cast_from(1.0_f64) + F::cast_from(0.29608574643216675549e2_f64) / t270;
    let t274 = F::ln(t273);
    (t270, t273, t274)
}
