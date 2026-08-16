//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1202/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1202(t1378: f64, t32757: f64, t225: f64, t567: f64, t7722: f64, t214: f64, t1985: f64, t2015: f64, t7749: f64, t3887: f64, t26193: f64, t8458: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32758 = t1378 * t32757;
    let t32761 = t7722 * t225 * t567;
    let t32762 = t214 * t32761;
    let t32764 = 0.16449340668482264365e-1_f64 * t1985 * t32762;
    let t32765 = t2015 * t7749;
    let t32766 = t3887 * t32765;
    let t32769 = t26193 * t8458;
    (t32758, t32761, t32762, t32764, t32766, t32769)
}
