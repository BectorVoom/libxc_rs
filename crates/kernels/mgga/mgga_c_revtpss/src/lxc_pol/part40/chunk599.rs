//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 599/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk599<F: Float>(t2828: F, t868: F, t213: F, t2437: F, t2443: F, t2446: F, t2449: F, t2460: F, t2462: F, t2468: F, t2473: F, t257: F, t2761: F, t2765: F, t2772: F, t865: F, t887: F) -> (F, F) {
    let t2829 = t868 * t2828;
    let t2832 = t2437 - t2443 - 0.10975748638225852664e-1 * t2446 + 0.10975748638225852664e-1 * t2449 + t2460 + 0.19514881078765566038e-1 * t2462 - 0.19514881078765566038e-1 * t2468 - t2473 + 0.65854491829355115987e0 * t213 * t2761 * t257 - 0.13170898365871023197e1 * t2765 * t887 + 0.13170898365871023197e1 * t865 * t2772 - 0.65854491829355115987e0 * t865 * t2829;
    (t2829, t2832)
}
