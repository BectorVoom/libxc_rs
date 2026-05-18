//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 733/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk733<F: Float>(t609: F, t864: F, t2132: F, t7885: F, t448: F, t939: F, t2130: F, t862: F) -> (F, F, F, F, F) {
    let t7886 = t609 * t864;
    let t7887 = t2132 * t7886;
    let t7889 = F::new(0.26020884564615598386e1) * t7885 * t7887;
    let t7890 = t448 * t939;
    let t7896 = t862 * t2130;
    (t7886, t7887, t7889, t7890, t7896)
}
