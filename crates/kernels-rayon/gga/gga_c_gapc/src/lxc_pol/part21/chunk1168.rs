//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1168/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1168(t11321: f64, t4925: f64, t8950: f64, t11508: f64, t1749: f64, t3060: f64, t11325: f64, t8621: f64, t185: f64, t33643: f64, t11489: f64, t1038: f64, t152: f64, t1875: f64, t33722: f64, t5918: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34460 = t11321 * t4925 * t8950;
    let t34463 = t3060 * t11508 * t1749;
    let t34465 = t3060 * t11325;
    let t34466 = t34465 * t8621;
    let t34468 = t185 * t33643;
    let t34469 = t34468 * t11489;
    let t34474 = t1875 * t33722 * t1038 * t152 * t5918;
    (t34460, t34463, t34465, t34466, t34468, t34469, t34474)
}
