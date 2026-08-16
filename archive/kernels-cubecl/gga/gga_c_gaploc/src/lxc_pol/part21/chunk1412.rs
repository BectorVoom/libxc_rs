//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1412/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1412<F: Float>(t1959: F, t3730: F, t12270: F, t1955: F, t1961: F, t31480: F, t31483: F, t32091: F, t32093: F, t32095: F, t32099: F, t32723: F, t32731: F, t32734: F, t32736: F, t32740: F, t38458: F, t38869: F, t38872: F, t38874: F, t38876: F) -> F {
    let t38892 = t3730 * t1959;
    let t38897 = -F::cast_from(2.0_f64) * t12270 * t1955 + F::cast_from(2.0_f64) * t1961 * t38892 - t31480 - t31483 + t32091 + t32093 - t32095 - t32099 + t32723 + t32731 + t32734 - t32736 + t32740 + t38458 + t38869 - t38872 - t38874 + t38876;
    t38897
}
