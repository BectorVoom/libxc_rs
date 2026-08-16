//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1409/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1409(t3020: f64, t59157: f64, t8686: f64, t3018: f64, t59166: f64, t14852: f64, t5187: f64, t44181: f64, t5190: f64, t26248: f64, t8688: f64, t1460: f64, t52890: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t59176 = 0.57894567559743977359e3_f64 * t8686 * t59157 * t3020;
    let t59179 = 0.48245472966453314466e2_f64 * t3018 * t59166 * t3020;
    let t59181 = 6.0_f64 * t14852 * t5187;
    let t59183 = 0.96490945932906628932e2_f64 * t44181 * t5190;
    let t59186 = 0.620700176468474021e4_f64 * t26248 * t59157 * t8688;
    let t59188 = 4.0_f64 * t52890 * t1460;
    (t59176, t59179, t59181, t59183, t59186, t59188)
}
