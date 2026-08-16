//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1336/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1336(t10195: f64, t13784: f64, t2986: f64, t1887: f64, t2262: f64, t337: f64, t10186: f64, t10191: f64, t13783: f64, t984: f64, t10237: f64, t10277: f64, t343: f64) -> (f64, f64, f64, f64, f64) {
    let t42827 = t2986 * t13784 * t10195;
    let t42830 = t2262 * t337 * t1887;
    let t42833 = t10186 * t10191;
    let t42837 = t13783 * t984;
    let t42839 = t2986 * t42837 * t10237;
    let t42841 = t343 * t10277;
    (t42827, t42830, t42833, t42839, t42841)
}
