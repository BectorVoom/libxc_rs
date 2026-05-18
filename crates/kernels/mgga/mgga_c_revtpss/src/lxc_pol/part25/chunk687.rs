//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 687/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk687<F: Float>(t3999: F, t555: F, t1385: F, t1419: F, t1399: F, t1437: F, t213: F, t3924: F, t4004: F, t4057: F, t4066: F, t4082: F, t4085: F, t4090: F, t4094: F, t4099: F, t4105: F, t4109: F, t4113: F, t546: F, t820: F) -> (F, F, F) {
    let t4114 = t3999 * t555;
    let t4118 = t1385 * t1419;
    let t4131 = t4082 - t4085 + F::new(0.10975748638225852664e-1) * t4090 - F::new(0.10975748638225852664e-1) * t4094 + t4099 - F::new(0.19514881078765566038e-1) * t4105 + F::new(0.19514881078765566038e-1) * t4109 - t4113 + F::new(0.13170898365871023197e1) * t820 * t4114 * t4004 - F::new(0.13170898365871023197e1) * t820 * t4118 * t1399 - F::new(0.65854491829355115987e0) * t820 * t1437 * t4057 - F::new(0.65854491829355115987e0) * t820 * t1437 * t3924 + F::new(0.65854491829355115987e0) * t213 * t546 * t4066;
    (t4114, t4118, t4131)
}
