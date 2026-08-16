//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1002/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1002(t75231: f64, t75235: f64, t3219: f64, t9087: f64, t14639: f64, t2412: f64, t75238: f64, t75241: f64, t1614: f64, t3204: f64, t15489: f64, t16043: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77514 = 0.2553875993597870364e-4_f64 * t75231;
    let t77515 = 0.1702583995731913576e-4_f64 * t75235;
    let t77516 = t9087 * t3219;
    let t77517 = 0.42564599893297839398e-5_f64 * t77516;
    let t77518 = t2412 * t14639;
    let t77519 = 0.42564599893297839398e-5_f64 * t77518;
    let t77520 = 0.16263363996404810741e-4_f64 * t75238;
    let t77521 = 0.16263363996404810741e-4_f64 * t75241;
    let t77525 = t3204 * t1614;
    let t77528 = t16043 * t15489;
    (t77514, t77515, t77517, t77519, t77520, t77521, t77525, t77528)
}
