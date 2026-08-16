//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1413/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1413(t1959: f64, t3730: f64, t12270: f64, t1955: f64, t1961: f64, t31480: f64, t31483: f64, t32091: f64, t32093: f64, t32095: f64, t32099: f64, t32723: f64, t32731: f64, t32734: f64, t32736: f64, t32740: f64, t38458: f64, t38869: f64, t38872: f64, t38874: f64, t38876: f64) -> f64 {
    let t38892 = t3730 * t1959;
    let t38897 = -2.0_f64 * t12270 * t1955 + 2.0_f64 * t1961 * t38892 - t31480 - t31483 + t32091 + t32093 - t32095 - t32099 + t32723 + t32731 + t32734 - t32736 + t32740 + t38458 + t38869 - t38872 - t38874 + t38876;
    t38897
}
