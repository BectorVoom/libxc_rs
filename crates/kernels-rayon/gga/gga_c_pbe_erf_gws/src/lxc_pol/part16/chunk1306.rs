//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1306/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1306(t47184: f64, t52112: f64, t14849: f64, t804: f64, t12276: f64, t15102: f64, t321: f64, t1105: f64, t13756: f64, t14364: f64, t14383: f64, t14825: f64, t15101: f64, t2423: f64, t3946: f64, t4062: f64, t4066: f64, t52089: f64, t52113: f64, t52115: f64, t52127: f64, t8574: f64, t8759: f64) -> f64 {
    let t54832 = 6.0_f64 * t52112 * t47184;
    let t54843 = 6.0_f64 * t804 * t14849;
    let t54852 = 6.0_f64 * t52112 * t12276;
    let t54854 = 2.0_f64 * t321 * t15102;
    let t54858 = 3.0_f64 * t1105 * t3946 * t52089 + 6.0_f64 * t13756 * t4066 * t8759 - 6.0_f64 * t14364 * t14383 * t3946 - 6.0_f64 * t14364 * t14825 * t3946 - t15101 * t2423 * t4062 + 3.0_f64 * t3946 * t4066 * t8574 - 6.0_f64 * t52113 - 2.0_f64 * t52115 - t52127 - t54832 + t54843 - t54852 - t54854;
    t54858
}
