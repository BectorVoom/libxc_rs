//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 932/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk932(t1445: f64, t2949: f64, t813: f64, t9688: f64, t13130: f64, t2194: f64, t13157: f64, t4673: f64, t6060: f64, t13129: f64, t4614: f64, t3271: f64, t8556: f64) -> (f64, f64, f64, f64, f64) {
    let t43959 = 0.46011511144704899612e1_f64 * t813 * t1445 * t2949 * t9688;
    let t43961 = 0.46011511144704899612e1_f64 * t2194 * t13130;
    let t43972 = 0.14300195980740170667e1_f64 * t6060 * t4673 * t13157;
    let t43975 = 0.61348681526273199483e1_f64 * t813 * t4614 * t13129;
    let t43977 = 0.23833659967900284446e0_f64 * t3271 * t8556;
    (t43959, t43961, t43972, t43975, t43977)
}
