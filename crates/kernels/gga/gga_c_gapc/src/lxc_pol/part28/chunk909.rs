//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 909/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk909<F: Float>(t10316: F, t3230: F, t2225: F, t3198: F, t2217: F, t10284: F, t10290: F, t10295: F, t10299: F, t10303: F, t10306: F, t10311: F, t10314: F) -> (F, F, F, F) {
    let t10317 = t3230 * t10316;
    let t10319 = t2225 * t3198;
    let t10321 = t2217 * t3198;
    let t10323 = -F::cast_from(0.2087902056652481864e-5_f64) * t10284 + F::cast_from(0.58183124501243180478e-7_f64) * t10290 + F::cast_from(0.342503618217270647e-5_f64) * t10295 + F::cast_from(0.2087902056652481864e-5_f64) * t10299 + F::cast_from(0.11416787273909021566e-6_f64) * t10303 + F::cast_from(0.18788769913633132635e-4_f64) * t10306 + F::cast_from(0.33406432906439709826e-4_f64) * t10311 + F::cast_from(0.11742981196020707897e-5_f64) * t10314 + F::cast_from(0.82073827867876094584e-5_f64) * t10317 + F::cast_from(0.33406432906439709826e-4_f64) * t10319 - F::cast_from(0.77948343448359322927e-4_f64) * t10321;
    (t10317, t10319, t10321, t10323)
}
