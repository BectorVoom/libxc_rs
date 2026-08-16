//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 1003/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk1003(t10677: f64, t1445: f64, t2530: f64, t813: f64, t13157: f64, t4673: f64, t6060: f64, t13129: f64, t4614: f64, t3271: f64, t8556: f64, t2087: f64) -> (f64, f64, f64, f64, f64) {
    let t43968 = t813 * t1445 * t10677 * t2530;
    let t43972 = 0.14300195980740170667e1_f64 * t6060 * t4673 * t13157;
    let t43975 = 0.61348681526273199483e1_f64 * t813 * t4614 * t13129;
    let t43977 = 0.23833659967900284446e0_f64 * t3271 * t8556;
    let t43980 = 0.82820720060468819301e2_f64 * t2087 * t4614 * t13157;
    (t43968, t43972, t43975, t43977, t43980)
}
