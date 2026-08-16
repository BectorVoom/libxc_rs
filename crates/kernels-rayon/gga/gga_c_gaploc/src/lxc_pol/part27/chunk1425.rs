//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1425/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1425(t12161: f64, t2089: f64, t1445: f64, t2087: f64, t28381: f64, t33081: f64, t33084: f64, t33090: f64, t33092: f64, t33095: f64, t33098: f64, t33101: f64, t33105: f64, t33109: f64, t33112: f64, t33114: f64, t33117: f64, t33126: f64, t33127: f64, t723: f64) -> f64 {
    let t39027 = t2089 * t12161;
    let t39032 = t33081 - t33084 + t33090 + t33092 - 0.13803453343411469884e2_f64 * t2087 * t1445 * t39027 * t723 + t33095 - t33098 + t33101 - t33105 + t33109 - t33112 - t33114 + t33117 - t33126 - t33127 + t28381;
    t39032
}
