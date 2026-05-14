//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 635/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk635<F: Float>(t10459: F, t41: F, t10463: F, t702: F, t11226: F, t740: F, t5438: F, t791: F, t10501: F, t1992: F, t772: F, t10568: F, t5396: F, t760: F, t755: F, t10641: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11910 = t41 * t10459;
    let t11911 = t702 * t10463;
    let t11927 = t11226 * t740;
    let t11966 = 1.0 / t5438 / t791;
    let t11983 = 0.51588271604938271604e-3 * t10501;
    let t11984 = t1992 * t1992;
    let t11985 = 1.0 / t11984;
    let t11986 = t772 * t11985;
    let t12002 = 0.53272592592592592592e-1 * t10568;
    let t12017 = 1.0 / t5396 / t760;
    let t12018 = t755 * t12017;
    let t12042 = 0.16068111111111111111e1 * t10568;
    let t12043 = 0.46308888888888888888e0 * t10641;
    (t11910, t11911, t11927, t11966, t11983, t11986, t12002, t12018, t12042, t12043)
}
