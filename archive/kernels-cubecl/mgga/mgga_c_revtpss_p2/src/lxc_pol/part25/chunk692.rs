//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 692/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk692<F: Float>(t1343: F, t1353: F, t1450: F, t198: F, t3871: F, t3873: F, t3889: F, t4025: F, t4027: F, t4031: F, t4033: F, t4035: F, t4037: F, t4040: F, t4042: F, t4135: F, t4139: F, t4140: F, t4144: F, t4147: F, t532: F) -> F {
    let t4150 = t1450 * t198 * t4135 * t532 - t198 * t4144 * t4147 * t532 + F::cast_from(3.0_f64) * t1343 * t198 * t3889 + F::cast_from(6.0_f64) * t1353 * t4139 * t4140 + t3871 + t3873 + t4025 + t4027 + t4031 - t4033 - t4035 - t4037 - t4040 + t4042;
    t4150
}
