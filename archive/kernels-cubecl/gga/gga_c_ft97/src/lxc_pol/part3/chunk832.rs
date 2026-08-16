//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 832/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk832<F: Float>(t16955: F, t9127: F, t2210: F, t13153: F, t3446: F, t160: F, t4714: F, t379: F, t2221: F, t1882: F, t4726: F, t4805: F, t558: F) -> (F, F, F, F, F) {
    let t16956 = t9127 * t16955;
    let t16957 = t2210 * t16956;
    let t16960 = t13153 * t3446;
    let t16963 = t160 * t4714;
    let t16964 = t16963 * t379;
    let t16965 = t2221 * t16964;
    let t16969 = t1882 * t4726;
    let t16971 = t4805 * t558;
    (t16957, t16960, t16965, t16969, t16971)
}
