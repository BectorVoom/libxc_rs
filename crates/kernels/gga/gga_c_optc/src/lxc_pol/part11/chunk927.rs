//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 927/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk927<F: Float>(t110: F, t1789: F, t1793: F, t209: F, t508: F, t6432: F, t6435: F, t514: F, t535: F, t622: F, t1756: F, t1759: F, t6475: F, t6481: F, t115: F, t6568: F) -> (F, F, F, F, F, F) {
    let t21891 = 0.2291123905095794067e1 * t209 * t110 * t1789 * t1793;
    let t21895 = 0.68733717152873822009e1 * t209 * t508 * t6432 * t6435;
    let t21899 = 0.22161481481481481481e0 * t209 * t622 * t514 * t535;
    let t21903 = 0.28493333333333333334e0 * t209 * t110 * t1756 * t1759;
    let t21907 = 0.4274e0 * t209 * t6481 * t6475;
    let t21979 = t6568 * t115;
    (t21891, t21895, t21899, t21903, t21907, t21979)
}
