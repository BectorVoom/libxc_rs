//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2073/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2073(t13783: f64, t984: f64, t10277: f64, t343: f64, t42308: f64, t974: f64, t2978: f64, t698: f64, t2402: f64, t976: f64, t973: f64, t979: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42837 = t13783 * t984;
    let t42841 = t343 * t10277;
    let t42861 = t974 * t42308;
    let t42875 = t698 * t2978;
    let t42891 = t2402 * t976;
    let t42893 = t973 * t42891 * t979;
    (t42837, t42841, t42861, t42875, t42891, t42893)
}
