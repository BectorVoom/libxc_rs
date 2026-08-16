//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 936/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk936(t1301: f64, t3981: f64, t13614: f64, t397: f64, t403: f64, t396: f64, t12951: f64, t12830: f64, t3952: f64, t12924: f64, t1313: f64, t1312: f64) -> (f64, f64, f64, f64) {
    let t13868 = t1301 * t3981;
    let t13871 = t397 * t13614 * t403;
    let t13873 = 0.19989765240197019125e-1_f64 * t396 * t13871;
    let t13878 = t403 * t12951;
    let t13879 = t13878 * t12830;
    let t13880 = t3952 * t13879;
    let t13885 = t1313 * t12924;
    let t13886 = t1312 * t13885;
    (t13868, t13873, t13880, t13886)
}
