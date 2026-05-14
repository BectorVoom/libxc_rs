//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 742/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk742<F: Float>(t5985: F, t1964: F, t528: F, t1457: F, t164: F, t547: F, t762: F, t1597: F, t1464: F, t163: F, t169: F, t234: F, t366: F, t1479: F, t553: F, t535: F, t837: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5986 = 0.1186530987165140469e-3 * t5985;
    let t5988 = 0.94516221669423353502e-1 * t528 * t1964;
    let t5990 = t1457 * t164;
    let t5993 = 0.18903244333884670701e0 * t762 * t547;
    let t5996 = t1597 * t547;
    let t5999 = 0.189032443338846707e0 * t1464 * t164;
    let t6003 = 0.87811049408533800023e-1 * t169 * t366 * t234 * t163;
    let t6005 = 0.258995450979035416e-1 * t1479 * t553;
    let t6006 = t837 * t535;
    (t5986, t5988, t5990, t5993, t5996, t5999, t6003, t6005, t6006)
}
