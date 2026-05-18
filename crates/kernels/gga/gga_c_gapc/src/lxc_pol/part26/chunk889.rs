//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 889/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk889<F: Float>(t10316: F, t3230: F, t2225: F, t3198: F, t2217: F, t10284: F, t10290: F, t10295: F, t10299: F, t10303: F, t10306: F, t10311: F, t10314: F) -> F {
    let t10317 = t3230 * t10316;
    let t10319 = t2225 * t3198;
    let t10321 = t2217 * t3198;
    let t10323 = -F::new(0.2087902056652481864e-5) * t10284 + F::new(0.58183124501243180478e-7) * t10290 + F::new(0.342503618217270647e-5) * t10295 + F::new(0.2087902056652481864e-5) * t10299 + F::new(0.11416787273909021566e-6) * t10303 + F::new(0.18788769913633132635e-4) * t10306 + F::new(0.33406432906439709826e-4) * t10311 + F::new(0.11742981196020707897e-5) * t10314 + F::new(0.82073827867876094584e-5) * t10317 + F::new(0.33406432906439709826e-4) * t10319 - F::new(0.77948343448359322927e-4) * t10321;
    t10323
}
