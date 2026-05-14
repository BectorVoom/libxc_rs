//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 926/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk926<F: Float>(t6376: F, t699: F, t37200: F, t38570: F, t38608: F, t38610: F, t42666: F, t42685: F, t42693: F, t42696: F, t42697: F, t42698: F, t42702: F, t44866: F, t44874: F, t44878: F, t44882: F, t44886: F, t44888: F, t884: F) -> (F, F) {
    let t48217 = t699 * t6376;
    let t48225 = -t42666 - 0.1454648621559751559e0 * t38570 - 0.638468998399467591e-4 * t44866 - 0.60975299583150056624e-3 * t38608 + 0.60975299583150056624e-3 * t38610 + 0.59871208509319042821e-1 * t884 * t48217 + t42685 + t42693 - t42696 + t42697 + t42698 - t37200 - 0.5107751987195740728e-4 * t44874 + 0.15323255961587222184e-3 * t44878 - 0.20431007948782962912e-3 * t44882 + 0.24829349937757072983e-4 * t44886 - 0.39726959900411316773e-4 * t44888 - t42702;
    (t48217, t48225)
}
