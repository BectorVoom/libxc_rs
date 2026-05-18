//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 829/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk829<F: Float>(t6925: F, t810: F, t4545: F, t2474: F, t460: F, t40: F, t4757: F, t950: F, t1402: F, t34: F, t418: F, t532: F) -> (F, F, F, F, F, F) {
    let t6926 = t6925 * t810;
    let t6929 = F::new(0.12654485932329694421e1) * t4545;
    let t6930 = t2474 * t460;
    let t6931 = t40 * t6930;
    let t6932 = F::new(2.0) * t6931;
    let t6933 = t4757 * t950;
    let t6936 = t1402 * t34;
    let t6937 = t532 * t418;
    (t6926, t6929, t6932, t6933, t6936, t6937)
}
