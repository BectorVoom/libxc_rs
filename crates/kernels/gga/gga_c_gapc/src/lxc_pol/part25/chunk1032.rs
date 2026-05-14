//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1032/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1032<F: Float>(t33884: F, t7204: F, t11522: F, t18866: F, t9396: F, t11941: F, t9652: F, t3375: F, t33757: F, t33582: F, t3789: F, t11784: F, t11983: F, t3784: F, t3788: F, t7241: F) -> (F, F, F, F, F, F, F) {
    let t33932 = t7204 * t33884;
    let t33935 = t18866 * t11522 * t9396;
    let t33937 = t9652 * t11941;
    let t33939 = t33757 * t3375;
    let t33941 = t33582 * t3789;
    let t33943 = t11784 * t11983;
    let t33946 = t3784 * t7241 * t3788;
    (t33932, t33935, t33937, t33939, t33941, t33943, t33946)
}
