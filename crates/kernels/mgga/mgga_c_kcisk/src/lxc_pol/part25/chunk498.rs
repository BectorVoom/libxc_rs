//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 498/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk498<F: Float>(t1636: F, t1824: F, t4609: F, t1849: F, t708: F, t1876: F, t3290: F, t1877: F, t3293: F, t1646: F, t1648: F) -> (F, F, F, F, F, F) {
    let t4610 = t1636 * t1824;
    let t4611 = t4609 * t4610;
    let t4614 = t708 * t1849;
    let t4616 = t1876 * t4614 * t3290;
    let t4620 = t1876 * t1877 * t3293;
    let t4623 = t1646 * t708;
    let t4624 = t1648 * t1648;
    (t4610, t4611, t4616, t4620, t4623, t4624)
}
