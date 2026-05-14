//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1132/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1132<F: Float>(t4127: F, t8751: F, t13972: F, t15169: F, t1123: F, t2848: F, t331: F, t833: F, t850: F, t11576: F, t14423: F, t14682: F, t3989: F, t13859: F, t56296: F, t6287: F) -> (F, F, F, F, F) {
    let t56555 = t4127 * t8751;
    let t56560 = t13972 * t15169;
    let t56578 = t850 * t1123 * t2848 * t331 * t833;
    let t56582 = t3989 * t14682 * t14423 * t11576;
    let t56586 = t13859 * t14682 * t56296 * t6287;
    (t56555, t56560, t56578, t56582, t56586)
}
