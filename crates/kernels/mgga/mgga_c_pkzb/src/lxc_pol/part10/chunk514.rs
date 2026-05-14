//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 514/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk514<F: Float>(t2009: F, t301: F, t761: F, t758: F, t132: F, t747: F, t288: F, t749: F) -> (F, F, F, F, F) {
    let t2011 = t301 * t2009 * t761;
    let t2012 = t758 * t2011;
    let t2016 = 1.0 / t747 / t132;
    let t2018 = 1.0 / t749 / t288;
    let t2019 = t2016 * t2018;
    (t2011, t2012, t2016, t2018, t2019)
}
