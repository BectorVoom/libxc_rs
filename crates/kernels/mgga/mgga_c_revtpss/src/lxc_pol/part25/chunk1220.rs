//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1220/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1220<F: Float>(t25851: F, t4254: F, t1310: F, t25832: F, t651: F, t116: F, t25168: F, t1962: F, t41154: F, t11061: F, t30: F, t27383: F, t50066: F) -> (F, F, F, F, F, F) {
    let t92733 = F::new(6.0) * t4254 * t25851;
    let t92736 = F::new(6.0) * t651 * t1310 * t25832;
    let t92737 = t25168 * t116;
    let t92742 = t1962 * t41154;
    let t92743 = t30 * t11061;
    let t92747 = t27383 * t50066;
    (t92733, t92736, t92737, t92742, t92743, t92747)
}
