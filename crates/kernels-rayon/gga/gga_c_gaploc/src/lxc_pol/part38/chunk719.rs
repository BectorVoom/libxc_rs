//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 719/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk719(t13625: f64, t969: f64, t825: f64, t13591: f64, t13595: f64, t13597: f64, t13600: f64, t13604: f64, t13606: f64, t13608: f64, t13611: f64, t13613: f64, t13619: f64, t13623: f64, t2087: f64) -> (f64, f64) {
    let t13626 = t969 * t13625;
    let t13627 = t825 * t13626;
    let t13629 = t13591 - t13595 + t13597 + t13600 - t13604 - t13606 - t13608 + t13611 - 0.13803453343411469884e2_f64 * t2087 * t13613 + t13619 - t13623 - 0.38342925953920749677e0_f64 * t13627;
    (t13626, t13629)
}
