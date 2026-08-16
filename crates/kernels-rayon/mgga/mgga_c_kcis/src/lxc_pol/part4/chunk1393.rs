//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1393/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1393(t12844: f64, t6172: f64, t4439: f64, t531: f64, t6183: f64, t833: f64, t4440: f64, t2645: f64, t6171: f64, t1444: f64, t2104: f64, t2642: f64) -> (f64, f64, f64, f64) {
    let t18091 = t12844 * t6172;
    let t18093 = t4439 * t18091 / 864.0_f64;
    let t18094 = t6183 * t531;
    let t18095 = t18094 * t833;
    let t18096 = t4440 * t18095;
    let t18099 = t6171 * t2645;
    let t18100 = t4440 * t18099;
    let t18103 = t2104 * t1444;
    let t18104 = t18103 * t2642;
    (t18093, t18096, t18100, t18104)
}
