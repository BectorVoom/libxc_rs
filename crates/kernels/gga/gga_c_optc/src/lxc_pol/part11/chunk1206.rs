//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1206/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1206<F: Float>(t4275: F, t5474: F, t14849: F, t15107: F, t28141: F, t47871: F, t21887: F, t21891: F, t21895: F, t21899: F, t21903: F, t21907: F, t28175: F, t28181: F, t37228: F, t37258: F) -> (F, F, F, F, F) {
    let t55797 = t5474 * t4275;
    let t55816 = t14849 * t15107;
    let t55862 = F::new(4.0) * t28141;
    let t55875 = F::new(4.0) * t47871;
    let t55876 = F::new(70.0) / F::new(3.0) * t37228 + F::new(140.0) / F::new(3.0) * t28175 - F::new(1820.0) / F::new(27.0) * t28181 + t21887 + t21891 + t21895 - t21899 - t21903 - t21907 - F::new(14.0) * t37258 + t55875;
    (t55797, t55816, t55862, t55875, t55876)
}
