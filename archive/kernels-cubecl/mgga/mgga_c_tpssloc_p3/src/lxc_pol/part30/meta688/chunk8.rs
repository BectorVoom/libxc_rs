//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2193/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2193<F: Float>(t5107: F, t652: F, t7467: F, t1774: F, t26135: F, t26179: F, t7461: F, t25980: F, t7458: F, t1983: F, t28826: F, t31299: F) -> (F, F, F, F, F) {
    let t97862 = F::cast_from(4.0_f64) * t652 * t5107 * t7467;
    let t97865 = F::cast_from(4.0_f64) * t652 * t1774 * t26135;
    let t97869 = F::cast_from(4.0_f64) * t26179 * t7461;
    let t97871 = F::cast_from(4.0_f64) * t7458 * t25980;
    let t97874 = F::cast_from(6.0_f64) * t1983 * t31299 * t28826;
    (t97862, t97865, t97869, t97871, t97874)
}
