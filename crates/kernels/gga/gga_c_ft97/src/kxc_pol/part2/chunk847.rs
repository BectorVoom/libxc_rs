//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 847/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk847<F: Float>(t2682: F, t992: F, t2875: F, t10492: F, t1882: F, t4305: F, t1255: F, t2862: F, t2739: F, t840: F, t4129: F, t882: F, t14889: F, t319: F, t1091: F, t2867: F) -> (F, F, F, F, F, F, F) {
    let t15200 = t992 * t2682;
    let t15201 = t2875 * t15200;
    let t15202 = t10492 * t15201;
    let t15206 = 2.0 / 9.0 * t1882 * t4305;
    let t15208 = t2862 * t1255 * t2682;
    let t15212 = t840 * t1255 * t2739;
    let t15218 = t840 * t882 * t4129;
    let t15222 = t840 * t319 * t14889;
    let t15225 = t1091 * t2867;
    (t15202, t15206, t15208, t15212, t15218, t15222, t15225)
}
