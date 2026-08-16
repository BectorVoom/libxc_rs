//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1010/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1010(t1312: f64, t14892: f64, t1591: f64, t3283: f64, t4400: f64, t13456: f64, t4406: f64, t4391: f64, t3952: f64, t1588: f64, t3532: f64, t3278: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14893 = t1312 * t14892;
    let t14896 = t3283 * t1591;
    let t14897 = t4400 * t14896;
    let t14898 = t1312 * t14897;
    let t14901 = t4406 * t13456;
    let t14902 = t1312 * t14901;
    let t14905 = t4391 * t13456;
    let t14906 = t3952 * t14905;
    let t14909 = t1588 * t3532;
    let t14910 = t3278 * t1591;
    (t14893, t14898, t14902, t14906, t14909, t14910)
}
