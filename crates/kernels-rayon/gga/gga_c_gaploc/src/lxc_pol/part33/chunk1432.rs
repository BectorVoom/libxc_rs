//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1432/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1432(t12223: f64, t1835: f64, t12153: f64, t12162: f64, t12163: f64, t12241: f64, t1457: f64, t1628: f64, t2004: f64, t2197: f64, t28726: f64, t33560: f64, t33564: f64, t33567: f64, t33569: f64, t33572: f64, t33574: f64, t33581: f64, t33584: f64, t33586: f64, t5666: f64, t833: f64) -> (f64, f64) {
    let t39166 = t12223 * t1835;
    let t39170 = 0.46011511144704899612e1_f64 * t2197 * t12163 + 0.61348681526273199482e1_f64 * t2197 * t12153 + 0.61348681526273199482e1_f64 * t833 * t1628 * t12162 - t33560 + t33564 + t33567 + t33569 + t33572 + t33574 + t33581 - t33584 + t33586 + 0.51123901271894332905e0_f64 * t5666 * t12241 + 0.35750489951850426669e0_f64 * t2004 * t1457 * t39166 - t28726;
    (t39166, t39170)
}
