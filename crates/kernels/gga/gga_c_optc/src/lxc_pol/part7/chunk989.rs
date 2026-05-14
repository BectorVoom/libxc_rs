//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 989/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk989<F: Float>(t1956: F, t2234: F, t6675: F, t732: F, t22410: F, t22417: F, t22434: F, t22439: F, t22646: F, t22648: F, t22652: F, t22655: F, t22657: F, t22659: F, t22661: F, t6569: F) -> (F, F, F, F) {
    let t23404 = t2234 * t1956;
    let t23406 = t732 * t6675;
    let t23409 = -t22410 + t22646 - t22648 - t22652 - t22655 - t22417 + t22434 - t22439 + t22657 - t22659 + t22661;
    let t23413 = 1820.0 / 27.0 * t732 * t6569;
    (t23404, t23406, t23409, t23413)
}
