//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 526/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk526<F: Float>(t136: F, t555: F, t2457: F, t3964: F, t4086: F, t786: F, t1432: F, t1433: F, t2470: F, t3999: F, t198: F, t531: F) -> (F, F, F, F, F, F, F) {
    let t4096 = t555 * t136;
    let t4099 = F::new(0.11565819519348392139e-2) * t3964 * t4096 * t2457;
    let t4100 = t4086 * t555;
    let t4101 = t786 * t4100;
    let t4113 = F::new(0.13009920719177044025e-1) * t1432 * t1433 * t2470;
    let t4114 = t3999 * t555;
    let t4139 = t198 * t531;
    (t4096, t4099, t4100, t4101, t4113, t4114, t4139)
}
