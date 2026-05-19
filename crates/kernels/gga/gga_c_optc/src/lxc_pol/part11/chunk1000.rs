//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1000/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1000<F: Float>(t1785: F, t209: F, t6485: F, t110: F, t1789: F, t1793: F, t508: F, t6432: F, t6435: F, t514: F, t535: F, t622: F) -> (F, F, F, F) {
    let t21887 = F::cast_from(0.14246666666666666667e0_f64) * t209 * t6485 * t1785;
    let t21891 = F::cast_from(0.2291123905095794067e1_f64) * t209 * t110 * t1789 * t1793;
    let t21895 = F::cast_from(0.68733717152873822009e1_f64) * t209 * t508 * t6432 * t6435;
    let t21899 = F::cast_from(0.22161481481481481481e0_f64) * t209 * t622 * t514 * t535;
    (t21887, t21891, t21895, t21899)
}
