//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 529/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk529<F: Float>(t609: F, t4456: F, t4457: F, t286: F, t4390: F) -> (F, F, F) {
    let t614 = 0.0 < t609;
    let t4458 = t4456 * t4457;
    let t4459 = t286 * t4458;
    let t4463 = piecewise3(t614, t4390, -t4390);
    (t4458, t4459, t4463)
}
