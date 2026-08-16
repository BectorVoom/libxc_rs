//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 953/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk953(t230: f64, t2962: f64, t1049: f64, t678: f64, t5938: f64, t5940: f64, t5944: f64, t7567: f64, t7569: f64, t7572: f64, t7576: f64, t7578: f64, t7581: f64, t7584: f64, t7593: f64, t7595: f64, t7597: f64, t7599: f64) -> f64 {
    let t8439 = 8.0_f64 / 3.0_f64 * t2962 * t230;
    let t8440 = t1049 * t678;
    let t8442 = 0.43284165449459373508e0_f64 * t5938 + 0.1442805514981979117e0_f64 * t5940 - t5944 + t8439 + 8.0_f64 / 3.0_f64 * t8440 - t7567 - t7569 - t7572 - t7576 + t7578 - t7581 + t7584 + t7593 + t7595 + t7597 + t7599;
    t8442
}
