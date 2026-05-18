//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1193/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1193<F: Float>(t96217: F, t27811: F, t61287: F, t4981: F, t982: F, t990: F, t26757: F, t27832: F, t26714: F, t8030: F, t1009: F, t14395: F) -> (F, F, F, F, F, F) {
    let t96218 = F::new(0.22109259259259259258e-2) * t96217;
    let t96221 = t27811 * t61287;
    let t96227 = t4981 * t982 * t990;
    let t96231 = F::new(0.15445601851851851852e-3) * t27832 * t26757;
    let t96238 = F::new(0.46336805555555555556e-3) * t8030 * t26714;
    let t96241 = t14395 * t1009;
    (t96218, t96221, t96227, t96231, t96238, t96241)
}
