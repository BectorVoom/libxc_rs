//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1172/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1172(t20912: f64, t337: f64, t6560: f64, t2146: f64, t6535: f64, t6702: f64, t6258: f64, t6711: f64, t2293: f64, t6455: f64, t2262: f64, t359: f64, t362: f64) -> (f64, f64, f64, f64, f64) {
    let t20914 = t6560 * t337 * t20912;
    let t20916 = 3.0_f64 / 4.0_f64 * t2146 * t20914;
    let t20919 = t6702 * t6535 / 6.0_f64;
    let t20921 = t6711 * t6258 / 8.0_f64;
    let t20926 = t6455 * t2293;
    let t20930 = 1.0_f64 / t2262 / t359 * t362;
    (t20916, t20919, t20921, t20926, t20930)
}
