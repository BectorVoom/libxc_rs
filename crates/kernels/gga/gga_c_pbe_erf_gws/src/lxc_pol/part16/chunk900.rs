//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 900/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk900<F: Float>(t203: F, t7829: F, t184: F, t221: F, t1406: F, t181: F, t997: F, t562: F, t577: F, t5379: F, t1045: F, t1672: F) -> (F, F, F, F, F) {
    let t7830 = t203 * t7829;
    let t7831 = t7830 * t184;
    let t7833 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t7831 * t221;
    let t7834 = t1406 * t181;
    let t7835 = t7834 * t184;
    let t7837 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t7835 * t997;
    let t7838 = t562 * t577;
    let t7839 = t7838 * t184;
    let t7841 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t7839 * t997;
    let t7843 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t5379 * t997;
    let t7844 = t1672 * t1045;
    (t7833, t7837, t7841, t7843, t7844)
}
