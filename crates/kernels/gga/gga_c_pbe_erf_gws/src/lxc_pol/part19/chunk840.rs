//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 840/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk840<F: Float>(t156: F, t2885: F, t496: F, t1243: F, t2890: F, t2897: F, t501: F, t395: F, t1552: F, t978: F, t1251: F, t2863: F) -> (F, F, F, F, F, F) {
    let t8146 = t156 * t2885;
    let t8148 = t496 * t8146 / F::new(3.0);
    let t8149 = t2890 * t1243;
    let t8156 = t501 * t2897;
    let t8158 = F::new(0.146904e1) * t8156 * t395;
    let t8159 = t1552 * t978;
    let t8160 = t8159 * t1251;
    let t8197 = t2863 * t1243;
    (t8146, t8148, t8149, t8158, t8160, t8197)
}
