//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1083/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1083<F: Float>(t4620: F, t6956: F, t22889: F, t4616: F, t4727: F, t6766: F, t1310: F, t9430: F, t133: F, t193: F, t197: F, t4599: F) -> (F, F, F, F, F) {
    let t38770 = t6956 * t4620;
    let t38783 = t22889 * t4616;
    let t38910 = t4727 * t6766;
    let t38936 = t1310 * t9430;
    let t39007 = t193 * t133 * t4599 * t197;
    (t38770, t38783, t38910, t38936, t39007)
}
