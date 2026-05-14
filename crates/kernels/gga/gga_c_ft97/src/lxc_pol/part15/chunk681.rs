//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 681/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk681<F: Float>(t17198: F, t925: F, t2210: F, t20748: F, t3434: F, t2221: F, t20753: F, t9127: F, t1053: F, t17409: F, t144: F, t1017: F, t4839: F, t574: F, t1060: F, t2185: F, t4668: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t20926 = t17198 * t925;
    let t20927 = t2210 * t20926;
    let t20930 = t3434 * t20748;
    let t20931 = t2221 * t20930;
    let t20934 = t9127 * t20753;
    let t20935 = t2210 * t20934;
    let t20938 = t17409 * t1053;
    let t20939 = t144 * t20938;
    let t20942 = t574 * t4839 * t1017;
    let t20945 = t2185 * t1060 * t4668;
    (t20926, t20927, t20930, t20931, t20934, t20935, t20938, t20939, t20942, t20945)
}
