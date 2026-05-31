//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 944/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk944<F: Float>(t1372: F, t228: F, t1357: F, t2604: F, t1381: F, t2632: F, t2627: F, t922: F, t96: F, t2614: F, t3992: F, t1378: F, t4: F, t657: F) -> (F, F, F, F, F, F, F) {
    let t14837 = F::cast_from(32.0_f64) * t1372 * t228;
    let t14852 = t1357 * t2604;
    let t14854 = t1381 * t2632;
    let t14856 = t1381 * t2627;
    let t14866 = t96 * t922;
    let t14880 = t3992 * t2614;
    let t14883 = t1378 * t4 * t657;
    (t14837, t14852, t14854, t14856, t14866, t14880, t14883)
}
