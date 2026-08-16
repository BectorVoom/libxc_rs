//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 910/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk910(t486: f64, t16963: f64, t16964: f64, t1369: f64, t1377: f64, t1444: f64, t16349: f64, t1378: f64, t286: f64, t25: f64, t5733: f64, t493: f64, t3999: f64, t5732: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t495 = 0.0_f64 < t486;
    let t16965 = t16963 * t16964;
    let t16968 = t1369 * t1377;
    let t16969 = t16968 * t1444;
    let t16970 = t16969 * t16964;
    let t16974 = piecewise3(t495, t16349, -t16349);
    let t16975 = t1378 * t16974;
    let t16976 = t286 * t16975;
    let t16979 = t25 * t5733;
    let t16981 = t493 * t16979 / 144.0_f64;
    let t16984 = t3999 * t5732;
    (t16965, t16968, t16970, t16974, t16976, t16981, t16984)
}
