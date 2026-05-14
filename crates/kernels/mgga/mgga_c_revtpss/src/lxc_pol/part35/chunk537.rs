//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 537/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk537<F: Float>(t1580: F, t213: F, t2437: F, t2443: F, t2460: F, t2473: F, t257: F, t4323: F, t4326: F, t4474: F, t4478: F, t4482: F, t6042: F, t6049: F, t6072: F, t865: F) -> (F,) {
    let t6075 = t2437 - t2443 - 0.10975748638225852664e-1 * t4323 + 0.10975748638225852664e-1 * t4478 + t2460 + 0.19514881078765566038e-1 * t4326 - 0.19514881078765566038e-1 * t4482 - t2473 + 0.65854491829355115987e0 * t213 * t6042 * t257 - 0.13170898365871023197e1 * t4474 * t1580 + 0.13170898365871023197e1 * t865 * t6049 - 0.65854491829355115987e0 * t865 * t6072;
    (t6075,)
}
