//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 561/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk561(t1003: f64, t922: f64, t2894: f64, t1071: f64, t291: f64, t2630: f64, t993: f64, t2635: f64, t994: f64, t290: f64, t999: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2895 = t922 * t1003;
    let t2896 = t2894 * t2895;
    let t2899 = t291 * t1071;
    let t2900 = t2899 * t2630;
    let t2901 = t993 * t2900;
    let t2904 = t994 * t2635;
    let t2905 = t993 * t2904;
    let t2909 = 1.0_f64 / t999 / t290;
    (t2895, t2896, t2900, t2901, t2904, t2905, t2909)
}
