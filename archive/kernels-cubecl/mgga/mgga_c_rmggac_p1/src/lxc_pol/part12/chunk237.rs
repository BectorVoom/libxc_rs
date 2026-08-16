//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 237/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk237<F: Float>(t446: F, t500: F, t385: F, t422: F, t388: F, t421: F, t155: F, t389: F, t1002: F, t1004: F, t1011: F, t1014: F, t1017: F, t1019: F, t1021: F, t1022: F, t436: F, t948: F, t975: F, t982: F, t998: F) -> (F, F, F, F, F, F, F, F) {
    let t1023 = t500 * t446;
    let t1027 = F::cast_from(8.0_f64) * t385 * t422;
    let t1028 = t388 * t421;
    let t1029 = t155 * t1028;
    let t1030 = F::cast_from(2.0_f64) * t1029;
    let t1031 = t385 * t389;
    let t1032 = F::cast_from(8.0_f64) * t1031;
    let t1033 = t948 - t975 + t982 + F::cast_from(0.93273e-1_f64) * t436 * t998 - F::cast_from(0.31091e-1_f64) * t1002 * t1004 + t1011 + t1014 - t1017 + t1019 + t1021 + F::cast_from(0.186546e0_f64) * t1022 * t1023 - t1027 + t1030 - t1032;
    (t1023, t1027, t1028, t1029, t1030, t1031, t1032, t1033)
}
