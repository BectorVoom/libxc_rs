//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1266/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1266(t54305: f64, t54352: f64, t54356: f64, t54381: f64, t54427: f64, t54621: f64, t54641: f64, t54719: f64, t54724: f64, t1167: f64, t2494: f64, t1105: f64, t3324: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t55582 = 119.0_f64 / 1728.0_f64 * t54305;
    let t55607 = 119.0_f64 / 864.0_f64 * t54352;
    let t55609 = 35.0_f64 / 108.0_f64 * t54356;
    let t55623 = 35.0_f64 / 216.0_f64 * t54381;
    let t55751 = 119.0_f64 / 1728.0_f64 * t54427;
    let t55892 = 35.0_f64 / 216.0_f64 * t54621;
    let t55947 = 35.0_f64 / 216.0_f64 * t54641;
    let t55984 = 35.0_f64 / 108.0_f64 * t54719;
    let t55986 = 119.0_f64 / 6912.0_f64 * t54724;
    let t56018 = t2494 * t1167;
    let t56027 = t1105 * t3324;
    (t55582, t55607, t55609, t55623, t55751, t55892, t55947, t55984, t55986, t56018, t56027)
}
