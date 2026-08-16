//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 727/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk727(t23384: f64, t7554: f64, t7607: f64, t225: f64, t7577: f64, t23665: f64, t7611: f64, t1625: f64, t362: f64, t6743: f64, t7614: f64, t968: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25450 = t23384 * t7554;
    let t25465 = t23384 * t7607;
    let t25470 = t7577 * t225;
    let t25508 = t23665 * t7611;
    let t25516 = t362 * t1625;
    let t25523 = t7577 * t6743;
    let t25529 = t968 * t7614;
    (t25450, t25465, t25470, t25508, t25516, t25523, t25529)
}
