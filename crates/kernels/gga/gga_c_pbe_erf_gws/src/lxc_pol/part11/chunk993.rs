//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 993/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk993<F: Float>(t2242: F, t3909: F, t3780: F, t4394: F, t20839: F, t3816: F, t1114: F, t3747: F, t6643: F, t3916: F, t6644: F, t11609: F, t2118: F) -> (F, F, F, F, F, F) {
    let t36340 = t2242 * t3909;
    let t36612 = t3780 * t4394;
    let t36626 = t20839 * t3816;
    let t36641 = t1114 * t3747 * t6643;
    let t36659 = t3916 * t6644;
    let t36666 = t2118 * t11609;
    (t36340, t36612, t36626, t36641, t36659, t36666)
}
