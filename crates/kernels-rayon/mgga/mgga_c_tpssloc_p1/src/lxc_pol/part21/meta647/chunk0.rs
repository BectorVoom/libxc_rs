//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2441/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2441(t340: f64, t625: f64, t221: f64, t339: f64, t344: f64, t1887: f64, t2262: f64, t337: f64, t13783: f64, t984: f64, t10277: f64, t343: f64) -> (f64, f64, f64, f64, f64) {
    let t42813 = t625 * t340;
    let t42817 = 0.82304526748971193413e-3_f64 * t339 * t221 * t42813 * t344;
    let t42830 = t2262 * t337 * t1887;
    let t42837 = t13783 * t984;
    let t42841 = t343 * t10277;
    (t42813, t42817, t42830, t42837, t42841)
}
