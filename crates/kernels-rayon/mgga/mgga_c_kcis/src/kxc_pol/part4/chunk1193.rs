//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1193/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1193(t15235: f64, t3515: f64, t1259: f64, t4951: f64, t1262: f64, t4621: f64, t13475: f64, t5310: f64, t1071: f64, t1851: f64, t2630: f64, t13495: f64) -> (f64, f64, f64, f64, f64) {
    let t15236 = t3515 * t15235;
    let t15239 = t4951 * t1259;
    let t15240 = t4621 * t1262;
    let t15241 = t15239 * t15240;
    let t15244 = t5310 * t13475;
    let t15247 = t1851 * t1071;
    let t15248 = t15247 * t2630;
    let t15249 = t3515 * t15248;
    let t15252 = t5310 * t13495;
    (t15236, t15241, t15244, t15249, t15252)
}
