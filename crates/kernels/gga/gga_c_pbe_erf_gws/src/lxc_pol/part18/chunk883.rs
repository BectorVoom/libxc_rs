//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 883/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk883<F: Float>(t11019: F, t1620: F, t3406: F, t5137: F, t639: F, t3554: F, t582: F, t211: F, t2601: F, t2722: F, t1621: F, t3478: F, t586: F, t645: F, t11004: F, t11009: F, t11014: F, t11016: F, t11018: F, t7852: F, t7870: F, t7873: F, t7876: F, t7880: F, t7890: F, t7905: F) -> (F, F, F, F, F, F) {
    let t11020 = t1620 * t11019;
    let t11021 = 32.0 / 135.0 * t11020;
    let t11022 = t5137 * t3406;
    let t11023 = t639 * t11022;
    let t11024 = 16.0 / 135.0 * t11023;
    let t11025 = t582 * t3554;
    let t11026 = t211 * t11025;
    let t11027 = 4.0 / 45.0 * t11026;
    let t11028 = t2601 * t2722;
    let t11029 = t1621 * t11028;
    let t11031 = 8.0 / 15.0 * t639 * t11029;
    let t11032 = t3478 * t586;
    let t11034 = 4.0 / 45.0 * t11032 * t645;
    let t11035 = t11004 + t7852 + t7870 - t7873 - t7876 + t7880 + t7890 + t11009 + t11014 + t11016 - t7905 - t11018 + t11021 - t11024 - t11027 + t11031 + t11034;
    (t11021, t11024, t11027, t11031, t11034, t11035)
}
