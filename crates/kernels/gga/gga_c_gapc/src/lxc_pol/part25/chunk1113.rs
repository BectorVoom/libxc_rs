//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1113/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1113<F: Float>(t11202: F, t128: F, t15354: F, t25054: F, t3643: F, t423: F, t11203: F, t8297: F, t11204: F, t25382: F, t1006: F, t125: F, t1552: F, t1954: F, t200: F, t11227: F, t8291: F) -> (F, F, F, F, F) {
    let t35489 = t11202 * t15354 * t128 * t25054;
    let t35491 = t3643 * t423;
    let t35493 = t35491 * t11203 * t8297;
    let t35495 = t11204 * t25382;
    let t35500 = t1006 * t125 * t1552 * t200 * t1954;
    let t35503 = t35491 * t11227 * t8291;
    (t35489, t35493, t35495, t35500, t35503)
}
