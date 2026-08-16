//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 553/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk553(t6076: f64, t8392: f64, t258: f64, t6061: f64, t1451: f64, t8232: f64, t1882: f64, t6105: f64, t1443: f64, t2492: f64) -> (f64, f64, f64, f64, f64) {
    let t24742 = t8392 * t6076;
    let t24747 = t258 * t6061;
    let t24757 = 4.0_f64 / 27.0_f64 * t8232 * t1451;
    let t24758 = t1882 * t6105;
    let t24789 = t2492 * t1443;
    (t24742, t24747, t24757, t24758, t24789)
}
