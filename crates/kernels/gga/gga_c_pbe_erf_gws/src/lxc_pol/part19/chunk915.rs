//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 915/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk915<F: Float>(t4785: F, t4791: F, t4796: F, t8004: F, t10018: F, t10019: F, t10022: F, t4652: F, t4664: F, t4751: F, t4784: F, t4790: F, t4799: F, t4803: F, t4807: F, t7994: F) -> (F, F, F, F, F) {
    let t10248 = F::cast_from(0.58482233974552040708e0_f64) * t4785;
    let t10249 = F::cast_from(0.17315755899375863299e2_f64) * t4791;
    let t10250 = F::cast_from(0.11696446794910408142e1_f64) * t4796;
    let t10251 = F::cast_from(0.48830813431341759843e-3_f64) * t8004;
    let t10252 = t4751 + t4652 - t7994 + t10018 + t4664 + t10019 - t10022 - t4784 - t10248 - t4790 - t10249 + t10250 + t10251 - t4799 - t4803 + t4807;
    (t10248, t10249, t10250, t10251, t10252)
}
