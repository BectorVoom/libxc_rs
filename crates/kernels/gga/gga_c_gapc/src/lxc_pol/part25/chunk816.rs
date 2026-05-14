//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 816/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk816<F: Float>(t10309: F, t2531: F, t799: F, t2493: F, t435: F, t3243: F, t2316: F, t493: F, t3230: F, t2225: F, t3198: F, t2217: F, t10284: F, t10290: F, t10295: F, t10299: F, t10303: F, t10306: F) -> (F, F, F, F, F, F) {
    let t10310 = t10309 * t2531;
    let t10311 = t799 * t10310;
    let t10313 = t435 * t2493;
    let t10314 = t3243 * t10313;
    let t10316 = t493 * t2316;
    let t10317 = t3230 * t10316;
    let t10319 = t2225 * t3198;
    let t10321 = t2217 * t3198;
    let t10323 = -0.2087902056652481864e-5 * t10284 + 0.58183124501243180478e-7 * t10290 + 0.342503618217270647e-5 * t10295 + 0.2087902056652481864e-5 * t10299 + 0.11416787273909021566e-6 * t10303 + 0.18788769913633132635e-4 * t10306 + 0.33406432906439709826e-4 * t10311 + 0.11742981196020707897e-5 * t10314 + 0.82073827867876094584e-5 * t10317 + 0.33406432906439709826e-4 * t10319 - 0.77948343448359322927e-4 * t10321;
    (t10311, t10314, t10317, t10319, t10321, t10323)
}
