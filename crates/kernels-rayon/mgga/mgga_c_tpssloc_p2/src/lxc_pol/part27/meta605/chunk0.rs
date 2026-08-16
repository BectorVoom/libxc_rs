//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2076/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2076(t6790: f64, t82632: f64, t6787: f64, t225: f64, t23547: f64, t23631: f64, t974: f64, t976: f64, t984: f64, t1009: f64, t343: f64, t25490: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82633 = t82632 * t6790;
    let t82635 = t82632 * t6787;
    let t82643 = t23547 * t225;
    let t82653 = t23631 * t974 * t976 * t984;
    let t82654 = t343 * t1009;
    let t82655 = t82654 * t25490;
    (t82633, t82635, t82643, t82653, t82654, t82655)
}
