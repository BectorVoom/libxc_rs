//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 382/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk382<F: Float>(t235: F, t693: F, t226: F, t209: F, t625: F, t228: F, t231: F, t173: F, t705: F, t701: F, t191: F, t668: F) -> (F, F, F, F, F, F, F) {
    let t2426 = F::cast_from(1.0_f64) / t693 / t235;
    let t2427 = t226 * t2426;
    let t2432 = t209 * t625;
    let t2434 = t228 * t2432 * t231;
    let t2435 = F::cast_from(0.42562405586419753087e-2_f64) * t2434;
    let t2436 = t173 * t705;
    let t2437 = t701 * t2436;
    let t2440 = F::cast_from(1.0_f64) / t191 / t668;
    (t2426, t2427, t2434, t2435, t2436, t2437, t2440)
}
