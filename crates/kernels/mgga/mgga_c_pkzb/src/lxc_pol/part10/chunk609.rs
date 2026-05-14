//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 609/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk609<F: Float>(t2422: F, t2430: F, t2454: F, t411: F, t415: F, t938: F, t952: F, t955: F, t418: F) -> (F, F, F, F) {
    let t2457 = 0.65854491829355115987e0 * t2422 * t415 - 0.13170898365871023197e1 * t938 * t952 + 0.13170898365871023197e1 * t411 * t2430 - 0.65854491829355115987e0 * t411 * t2454;
    let t2461 = t955 * t955;
    let t2463 = t418 * t418;
    let t2464 = 1.0 / t2463;
    (t2457, t2461, t2463, t2464)
}
