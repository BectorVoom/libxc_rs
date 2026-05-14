//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 371/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk371<F: Float>(t1589: F, t1591: F, t1586: F, t1568: F, t1572: F, t1578: F, t1580: F, t1583: F, t535: F, t541: F) -> (F, F, F) {
    let t1592 = t1589 * t1591;
    let t1593 = t1586 * t1592;
    let t1596 = 0.2698618307426597582e-1 * t1568 * t541 - 0.71963154864709268853e-1 * t1572 * t541 + t1578 + 0.89953943580886586067e-2 * t1580 * t1583 - 0.2698618307426597582e-1 * t535 * t1593;
    (t1592, t1593, t1596)
}
