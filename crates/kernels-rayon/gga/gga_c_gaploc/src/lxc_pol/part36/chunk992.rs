//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 992/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk992(t10930: f64, t10931: f64, t43486: f64, t23220: f64, t43598: f64, t43683: f64, t7572: f64, t7573: f64, t43494: f64, t7427: f64, t10667: f64, t2033: f64, t2365: f64, t2610: f64) -> (f64, f64, f64, f64, f64) {
    let t43800 = 0.55213813373645879534e2_f64 * t10930 * t10931 * t43486;
    let t43803 = 0.27606906686822939767e2_f64 * t23220 * t10931 * t43598;
    let t43806 = 0.69017266717057349418e1_f64 * t7572 * t7573 * t43683;
    let t43809 = 0.37959496694381542179e3_f64 * t7427 * t7573 * t43494;
    let t43812 = t2033 * t2365 * t2610 * t10667;
    (t43800, t43803, t43806, t43809, t43812)
}
