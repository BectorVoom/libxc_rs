//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 714/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk714<F: Float>(t305: F, t9957: F, t793: F, t9765: F, t797: F, t9999: F, t2068: F, t9873: F, t7829: F, t9889: F, t1763: F, t36: F) -> (F, F, F, F, F, F) {
    let t10154 = t305 * t9957;
    let t10156 = t793 * t9765;
    let t10158 = t797 * t9999;
    let t10162 = t2068 * t9873;
    let t10164 = t7829 * t9889;
    let t10166 = t36 * t1763;
    (t10154, t10156, t10158, t10162, t10164, t10166)
}
