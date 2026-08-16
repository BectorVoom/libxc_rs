//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1375/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1375(t12985: f64, t2586: f64, t2570: f64, t67: f64, t792: f64, t12984: f64, t686: f64, t776: f64, t131: f64, t9558: f64, t205: f64, t1489: f64, t9541: f64) -> (f64, f64, f64, f64) {
    let t12986 = t2586 * t12985;
    let t12997 = t2570 * t67;
    let t12998 = t792 * t12997;
    let t13000 = t686 * t12984 * t776;
    let t13002 = 0.49999999999999999998e-2_f64 * t12998 * t13000;
    let t13004 = t9558 * t131;
    let t13005 = t205 * t13004;
    let t13010 = t9541 * t1489;
    (t12986, t13002, t13005, t13010)
}
