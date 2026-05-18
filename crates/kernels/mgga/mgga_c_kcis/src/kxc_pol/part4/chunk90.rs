//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 90/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk90<F: Float>(t242: F, t245: F, t248: F, t255: F) -> (F, F, F) {
    let t270 = F::new(0.51785e1) * t245 + F::new(0.905775e0) * t242 + F::new(0.1100325e0) * t248 + F::new(0.1241775e0) * t255;
    let t273 = F::new(1.0) + F::new(0.29608574643216675549e2) / t270;
    let t274 = f64::ln(t273);
    (t270, t273, t274)
}
