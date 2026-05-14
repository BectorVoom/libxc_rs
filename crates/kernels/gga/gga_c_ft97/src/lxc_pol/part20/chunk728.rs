//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 728/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk728<F: Float>(t1882: F, t4178: F, t4183: F, t1255: F, t2413: F, t835: F, t2405: F, t2857: F, t10447: F, t4151: F, t14116: F, t4140: F, t4139: F, t2409: F, t4145: F, t2874: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15500 = 2.0 / 9.0 * t1882 * t4178;
    let t15502 = 4.0 / 9.0 * t1882 * t4183;
    let t15504 = t835 * t1255 * t2413;
    let t15508 = t2857 * t1255 * t2405;
    let t15511 = t10447 * t4151;
    let t15514 = t4140 * t14116;
    let t15515 = t4139 * t15514;
    let t15518 = t4145 * t2409;
    let t15519 = t2874 * t15518;
    (t15500, t15502, t15504, t15508, t15511, t15514, t15515, t15518, t15519)
}
