//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 596/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk596(t4685: f64, t951: f64, t1680: f64, t2933: f64, t949: f64, t2938: f64, t1670: f64, t2960: f64, t934: f64, t4625: f64, t939: f64, t1676: f64, t659: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4687 = 1.0_f64 * t4685 * t951;
    let t4689 = 1.0_f64 * t2933 * t1680;
    let t4690 = t1680 * t949;
    let t4692 = 2.0_f64 * t2938 * t4690;
    let t4700 = t2960 * t1670;
    let t4701 = t4700 * t934;
    let t4703 = t939 * t4625;
    let t4706 = t659 * t1676;
    (t4687, t4689, t4690, t4692, t4700, t4701, t4703, t4706)
}
