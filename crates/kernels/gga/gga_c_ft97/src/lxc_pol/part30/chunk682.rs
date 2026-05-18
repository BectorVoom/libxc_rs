//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 682/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk682<F: Float>(t28859: F, t875: F, t7022: F, t880: F, t193: F, t1253: F, t824: F, t6222: F, t681: F, t7023: F, t28491: F, t28494: F, t28499: F, t28504: F, t28509: F, t28514: F, t28518: F, t28522: F, t28526: F, t28529: F, t28531: F, t28536: F) -> (F, F, F, F, F, F) {
    let t28860 = t28859 * t875;
    let t28862 = t7022 * t880;
    let t28863 = t193 * t28862;
    let t28868 = t1253 * t824;
    let t28869 = t6222 * t28868;
    let t28870 = t193 * t28869;
    let t28873 = t681 * t7023;
    let t28885 = t28491 / F::new(3.0) - t28494 / F::new(12.0) + t28499 + t28504 + t28509 + t28514 / F::new(4.0) - F::new(2.0) / F::new(3.0) * t28518 - F::new(2.0) / F::new(3.0) * t28522 + F::new(2.0) / F::new(9.0) * t28526 + t28529 / F::new(6.0) - t28531 / F::new(9.0) + t28536 / F::new(3.0);
    (t28860, t28863, t28868, t28870, t28873, t28885)
}
