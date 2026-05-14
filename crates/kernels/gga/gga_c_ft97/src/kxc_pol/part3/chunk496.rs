//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 496/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk496<F: Float>(t1234: F, t2755: F, t856: F, t91: F, t1228: F, t1775: F, t2: F, t2766: F, t3691: F, t2771: F, t4037: F, t848: F, t3700: F, t3921: F, t1232: F, t458: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4191 = t2755 * t1234;
    let t4193 = t91 * t4191 * t856;
    let t4197 = t1775 * t1228;
    let t4199 = t2766 * t2;
    let t4200 = t4199 * t3691;
    let t4203 = t2771 * t4037;
    let t4206 = t848 * t2;
    let t4207 = t4206 * t3700;
    let t4210 = t848 * t3921;
    let t4213 = t458 * t1232;
    (t4191, t4193, t4197, t4199, t4200, t4203, t4206, t4207, t4210, t4213)
}
