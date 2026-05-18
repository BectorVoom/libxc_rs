//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 979/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk979<F: Float>(t169: F, t301: F, t4867: F, t784: F, t159: F, t18075: F, t285: F, t2331: F, t274: F, t1473: F, t1488: F, t1492: F) -> (F, F, F, F, F) {
    let t18116 = t169 * t784 * t4867 * t301;
    let t18122 = F::new(0.3831185177913978998e-1) * t18075 * t159 * t285;
    let t18126 = F::new(0.52404510650723236824e1) * t169 * t2331 * t274 * t301;
    let t18129 = t1473 * t1488;
    let t18131 = t1473 * t1492;
    (t18116, t18122, t18126, t18129, t18131)
}
