//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1066/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1066<F: Float>(t11937: F, t2639: F, t11889: F, t16408: F, t612: F, t11887: F, t7956: F, t818: F, t9066: F, t11986: F, t7939: F, t190: F, t8785: F) -> (F, F, F, F, F) {
    let t33245 = t11937 * t2639;
    let t33248 = t16408 * t612 * t11889;
    let t33252 = t11887 * t9066 * t818 * t7956;
    let t33254 = t11986 * t7939;
    let t33256 = t190 * t8785;
    (t33245, t33248, t33252, t33254, t33256)
}
