//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1014/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1014<F: Float>(t16156: F, t9194: F, t9190: F, t1001: F, t236: F, t3351: F, t35312: F, t551: F, t27111: F, t515: F, t9188: F, t9184: F) -> (F, F, F, F, F) {
    let t42204 = t16156 * t9194;
    let t42206 = t16156 * t9190;
    let t42211 = t3351 * t35312 * t236 * t551 * t1001;
    let t42215 = t3351 * t9188 * t515 * t27111;
    let t42217 = t16156 * t9184;
    (t42204, t42206, t42211, t42215, t42217)
}
