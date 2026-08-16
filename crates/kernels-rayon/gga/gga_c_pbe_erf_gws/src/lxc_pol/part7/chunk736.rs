//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 736/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk736(t1492: f64, t751: f64, t1497: f64, t6032: f64, t6036: f64, t6039: f64, t6043: f64, t6049: f64, t6050: f64, t6053: f64, t6058: f64, t6059: f64) -> f64 {
    let t6061 = t751 * t1492;
    let t6064 = 0.59871170051273045469e-1_f64 * t751 * t1497;
    let t6065 = -t6032 - t6036 - 0.54655730795145295329e-4_f64 * t6039 - t6043 + t6049 - 0.15965645347006145458e0_f64 * t6050 - t6053 - t6058 + 0.59871170051273045469e-1_f64 * t6059 + 0.11974234010254609094e0_f64 * t6061 + t6064;
    t6065
}
