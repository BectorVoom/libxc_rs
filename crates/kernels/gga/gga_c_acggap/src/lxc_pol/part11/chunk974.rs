//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 974/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk974<F: Float>(t137: F, t336: F, t4876: F, t578: F, t30916: F, t30918: F, t30921: F, t35040: F, t35042: F, t35043: F, t35047: F, t35052: F, t35055: F, t35059: F, t35062: F, t35065: F, t35068: F, t35071: F, t35073: F, t35075: F, t35076: F) -> (F,) {
    let t35080 = t578 * t336 * t4876 * t137;
    let t35082 = 0.85748036236139473944e-3 * t30916 + t35040 + t35042 - 35.0 / 216.0 * t35043 - 0.10718504529517434243e-3 * t35047 - t35052 - 0.7862023072401038017e-3 * t35055 + 0.47172138434406228102e-3 * t30918 - t35059 / 16.0 - t35062 / 16.0 - 0.22921875e-1 * t35065 - 0.4584375e-1 * t35068 - t35071 - t35073 - t35075 - t30921 - 77.0 / 576.0 * t35076 - 0.38203125e-2 * t35080;
    (t35082,)
}
