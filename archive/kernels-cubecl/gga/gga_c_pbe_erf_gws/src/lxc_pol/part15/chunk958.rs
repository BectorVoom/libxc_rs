//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 958/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk958<F: Float>(t7800: F, t7801: F, t7803: F, t7805: F, t7806: F, t7808: F, t7810: F, t7812: F, t7833: F, t7837: F, t7841: F, t7843: F, t7846: F, t7848: F, t7850: F, t7852: F, t7856: F) -> F {
    let t8453 = t7800 + t7801 - t7803 - t7805 - t7806 - t7808 - t7810 - t7812 + t7833 + t7837 + t7841 + t7843 + t7846 - t7848 - t7850 + t7852 + t7856;
    t8453
}
