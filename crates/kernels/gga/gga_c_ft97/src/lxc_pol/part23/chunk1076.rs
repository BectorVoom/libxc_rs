//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1076/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1076<F: Float>(t2378: F, t37481: F, t4977: F, t4939: F, t9608: F, t1095: F, t1127: F, t806: F, t17839: F, t3771: F, t6041: F, t1175: F, t2492: F, t18587: F, t761: F, t13927: F, t737: F) -> (F, F, F, F, F, F, F, F, F) {
    let t66422 = t37481 * t2378;
    let t66451 = t2378 * t4977;
    let t66493 = t9608 * t4939;
    let t66565 = t1095 * t1127;
    let t66633 = t806 * t4939;
    let t66680 = t3771 * t6041 * t17839;
    let t66735 = t2492 * t1175;
    let t67701 = t18587 * t761;
    let t67796 = t737 * t13927;
    (t66422, t66451, t66493, t66565, t66633, t66680, t66735, t67701, t67796)
}
