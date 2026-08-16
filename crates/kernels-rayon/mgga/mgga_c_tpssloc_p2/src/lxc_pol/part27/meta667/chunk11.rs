//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2354/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2354(t25971: f64, t83886: f64, t23831: f64, t4028: f64, t26504: f64, t6876: f64, t1983: f64, t7687: f64, t83929: f64, t1874: f64, t90370: f64, t26114: f64, t6525: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91578 = 6.0_f64 * t83886 * t25971;
    let t91580 = 2.0_f64 * t4028 * t23831;
    let t91582 = 2.0_f64 * t6876 * t26504;
    let t91585 = 3.0_f64 * t1983 * t83929 * t7687;
    let t91587 = 4.0_f64 * t90370 * t1874;
    let t91589 = 4.0_f64 * t26114 * t6525;
    (t91578, t91580, t91582, t91585, t91587, t91589)
}
