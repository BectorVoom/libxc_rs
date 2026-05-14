//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 235/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk235<F: Float>(t388: F, t421: F, t155: F, t385: F, t389: F, t1002: F, t1004: F, t1011: F, t1014: F, t1017: F, t1019: F, t1021: F, t1022: F, t1023: F, t1027: F, t436: F, t948: F, t975: F, t982: F, t998: F) -> (F, F, F, F, F, F) {
    let t1028 = t388 * t421;
    let t1029 = t155 * t1028;
    let t1030 = 2.0 * t1029;
    let t1031 = t385 * t389;
    let t1032 = 8.0 * t1031;
    let t1033 = t948 - t975 + t982 + 0.93273e-1 * t436 * t998 - 0.31091e-1 * t1002 * t1004 + t1011 + t1014 - t1017 + t1019 + t1021 + 0.186546e0 * t1022 * t1023 - t1027 + t1030 - t1032;
    (t1028, t1029, t1030, t1031, t1032, t1033)
}
