//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1225/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1225<F: Float>(t1306: F, t21027: F, t21030: F, t21033: F, t21037: F, t21039: F, t21291: F, t21299: F, t21301: F, t21306: F, t21308: F, t2153: F, t2993: F, t6065: F) -> F {
    let t21309 = F::cast_from(6.0_f64) * t1306 * t2153 * t2993 * t6065 - t21027 - t21030 - t21033 + t21037 + t21039 + t21291 - t21299 + t21301 - t21306 - t21308;
    t21309
}
