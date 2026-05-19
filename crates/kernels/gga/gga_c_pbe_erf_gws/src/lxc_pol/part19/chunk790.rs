//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 790/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk790<F: Float>(t164: F, t5984: F, t1964: F, t528: F, t1457: F, t547: F, t762: F, t1597: F, t1464: F, t163: F, t169: F, t234: F, t366: F) -> (F, F, F, F, F, F, F) {
    let t5985 = t5984 * t164;
    let t5986 = F::cast_from(0.1186530987165140469e-3_f64) * t5985;
    let t5988 = F::cast_from(0.94516221669423353502e-1_f64) * t528 * t1964;
    let t5990 = t1457 * t164;
    let t5993 = F::cast_from(0.18903244333884670701e0_f64) * t762 * t547;
    let t5996 = t1597 * t547;
    let t5999 = F::cast_from(0.189032443338846707e0_f64) * t1464 * t164;
    let t6003 = F::cast_from(0.87811049408533800023e-1_f64) * t169 * t366 * t234 * t163;
    (t5986, t5988, t5990, t5993, t5996, t5999, t6003)
}
