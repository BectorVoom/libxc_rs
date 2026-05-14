//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 521/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk521<F: Float>(t4597: F, t708: F, t3290: F, t4595: F, t1797: F, t574: F, t1636: F, t1648: F, t1876: F, t682: F, t1824: F, t1849: F, t1877: F, t3293: F, t1646: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4598 = t708 * t4597;
    let t4600 = t4595 * t4598 * t3290;
    let t4603 = t1797 * t574;
    let t4604 = t4603 * t708;
    let t4605 = t1636 * t1648;
    let t4606 = t4604 * t4605;
    let t4609 = t1876 * t682;
    let t4610 = t1636 * t1824;
    let t4611 = t4609 * t4610;
    let t4614 = t708 * t1849;
    let t4616 = t1876 * t4614 * t3290;
    let t4620 = t1876 * t1877 * t3293;
    let t4623 = t1646 * t708;
    let t4624 = t1648 * t1648;
    (t4598, t4600, t4604, t4606, t4609, t4611, t4616, t4620, t4623, t4624)
}
