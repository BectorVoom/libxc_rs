//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 383/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk383<F: Float>(t2: F, t848: F, t3700: F, t3921: F, t1232: F, t458: F, t2771: F, t4052: F, t1212: F, t2681: F, t824: F, t192: F, t4129: F, t852: F, t2761: F, t2762: F, t2764: F, t3139: F, t4197: F, t4200: F, t4203: F, t462: F, t92: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4206 = t848 * t2;
    let t4207 = t4206 * t3700;
    let t4210 = t848 * t3921;
    let t4213 = t458 * t1232;
    let t4215 = t2771 * t4052;
    let t4218 = t2 * t1212;
    let t4220 = t2681 * t4218 * t824;
    let t4224 = t192 * t852 * t4129;
    let t4226 = t2761 + t2762 / 9.0 + t2764 / 3.0 + t4197 / 9.0 - 2.0 / 9.0 * t462 * t4200 + t462 * t4203 / 3.0 + 2.0 / 3.0 * t462 * t4207 + 2.0 / 3.0 * t3139 * t4210 + t4213 / 3.0 + t462 * t4215 / 3.0 + 2.0 * t462 * t4220 - t92 * t4224;
    (t4206, t4207, t4210, t4213, t4215, t4218, t4220, t4224, t4226)
}
