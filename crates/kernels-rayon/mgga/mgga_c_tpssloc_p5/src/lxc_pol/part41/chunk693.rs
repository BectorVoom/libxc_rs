//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 693/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk693(t2990: f64, t4531: f64, t2824: f64, t3003: f64, t4384: f64, t4387: f64, t4390: f64, t4393: f64, t340: f64, t343: f64, t974: f64, t1597: f64, t984: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4532 = t4531 * t2990;
    let t4540 = -t3003 - t2824 / 9.0_f64 - t4384 / 9.0_f64 + t4387 / 18.0_f64 - t4390 / 3.0_f64 + t4393 / 6.0_f64;
    let t4541 = t340 * t4540;
    let t4542 = t4541 * t343;
    let t4543 = t974 * t4542;
    let t4546 = t974 * t340;
    let t4547 = t1597 * t984;
    (t4532, t4540, t4542, t4543, t4546, t4547)
}
