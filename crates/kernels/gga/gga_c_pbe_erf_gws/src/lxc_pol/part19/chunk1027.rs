//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1027/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1027<F: Float>(t14797: F, t3068: F, t3990: F, t3989: F, t3070: F, t3965: F, t3062: F, t3959: F, t1167: F, t810: F, t944: F, t1105: F, t14161: F, t2494: F, t4066: F, t4233: F, t945: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14799 = t3990 * t14797 * t3068;
    let t14800 = t3989 * t14799;
    let t14806 = t3965 * t3070;
    let t14812 = t3959 * t3062;
    let t14825 = t1167 * t810;
    let t14831 = t1167 * t944;
    let t14843 = t14161 * t1105;
    let t14849 = t4066 * t2494;
    let t14852 = t4233 * t945;
    (t14799, t14800, t14806, t14812, t14825, t14831, t14843, t14849, t14852)
}
