//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 760/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk760<F: Float>(t465: F, t4813: F, t1425: F, t409: F, t414: F, t1333: F, t461: F, t1438: F, t428: F, t1319: F, t456: F, t4607: F) -> (F, F, F, F, F, F, F) {
    let t4814 = t465 * t4813;
    let t4815 = F::new(0.56969282336565386482e-3) * t4814;
    let t4819 = t409 * t1425;
    let t4821 = t414 * t1425;
    let t4825 = t1333 * t461;
    let t4826 = F::new(60.0) * t4825;
    let t4827 = t1438 * t428;
    let t4830 = t1333 * t428;
    let t4835 = t1319 * t4607 * t456;
    (t4815, t4819, t4821, t4826, t4827, t4830, t4835)
}
