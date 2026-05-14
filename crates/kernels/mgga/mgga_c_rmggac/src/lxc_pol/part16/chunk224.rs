//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 224/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk224<F: Float>(t33: F, t77: F, t74: F, t184: F, t154: F, t73: F, t294: F, t384: F, t381: F, t422: F, t389: F, t195: F, t498: F, t446: F, t500: F, t385: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t1007 = t77 * t33;
    let t1008 = 1.0 / t1007;
    let t1009 = t74 * t1008;
    let t1011 = 20.0 * t1009 * t184;
    let t1012 = t73 * t154;
    let t1014 = 12.0 * t1012 * t184;
    let t1015 = t294 * t384;
    let t1017 = 32.0 * t1015 * t184;
    let t1019 = 8.0 * t381 * t422;
    let t1020 = t381 * t389;
    let t1022 = t195 * t498;
    let t1023 = t500 * t446;
    let t1027 = 8.0 * t385 * t422;
    (t1007, t1008, t1009, t1011, t1012, t1014, t1015, t1017, t1019, t1020, t1022, t1023, t1027)
}
