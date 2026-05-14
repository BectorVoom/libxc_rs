//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 999/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk999<F: Float>(t2472: F, t4780: F, t2529: F, t2517: F, t4863: F, t4919: F, t7504: F, t2492: F, t2619: F, t4963: F, t874: F, t4933: F, t530: F, t862: F, t4961: F, t7467: F) -> (F, F, F, F, F, F, F, F) {
    let t40919 = t4780 * t2472;
    let t40949 = t4780 * t2529;
    let t41291 = t4863 * t2517;
    let t41392 = t4919 * t7504;
    let t41396 = t4863 * t2492;
    let t41484 = t874 * t2619 * t4963;
    let t41498 = t862 * t530 * t4933;
    let t41521 = t7467 * t4961;
    (t40919, t40949, t41291, t41392, t41396, t41484, t41498, t41521)
}
