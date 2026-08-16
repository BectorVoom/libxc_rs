//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 901/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk901<F: Float>(t17899: F, t2379: F, t1096: F, t3750: F, t25: F, t5049: F, t3762: F, t5025: F, t5005: F, t1113: F, t3751: F, t3725: F) -> (F, F, F, F, F, F, F, F) {
    let t17900 = t2379 * t17899;
    let t17903 = t1096 * t3750;
    let t17904 = t2379 * t17903;
    let t17907 = t5049 * t25;
    let t17908 = t17907 * t3762;
    let t17911 = t5025 * t25;
    let t17912 = t17911 * t3762;
    let t17915 = t5005 * t25;
    let t17916 = t17915 * t3762;
    let t17919 = t3751 * t1113;
    let t17923 = t3725 * t1113;
    (t17900, t17903, t17904, t17908, t17912, t17916, t17919, t17923)
}
