//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 646/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk646<F: Float>(t10010: F, t739: F, t236: F, t6108: F, t1971: F, t7365: F, t6182: F, t1970: F, t209: F, t558: F, t605: F, t511: F, t570: F, t515: F, t8443: F, t8451: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10011 = t739 * t10010;
    let t10012 = 0.23948483403727617128e0 * t10011;
    let t10013 = t236 * t6108;
    let t10014 = t1971 * t10013;
    let t10015 = t7365 * t10014;
    let t10016 = 0.85129199786595678796e-5 * t10015;
    let t10017 = t236 * t6182;
    let t10018 = t1971 * t10017;
    let t10019 = t1970 * t10018;
    let t10020 = 0.42564599893297839398e-5 * t10019;
    let t10022 = t558 * t605 * t209;
    let t10023 = t511 * t10022;
    let t10024 = t1971 * t10023;
    let t10025 = t1970 * t10024;
    let t10026 = 0.25538759935978703638e-4 * t10025;
    let t10028 = t570 * t605 * t209;
    let t10029 = t515 * t10028;
    let t10030 = t1971 * t10029;
    let t10031 = t1970 * t10030;
    let t10032 = 0.85129199786595678796e-5 * t10031;
    let t10033 = t8451 * t8443;
    (t10012, t10014, t10016, t10018, t10020, t10024, t10026, t10030, t10032, t10033)
}
