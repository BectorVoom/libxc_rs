//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 725/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk725<F: Float>(t3521: F, t4611: F, t1814: F, t1876: F, t1636: F, t4658: F, t1824: F, t4644: F, t4609: F, t1646: F, t1797: F, t708: F) -> (F, F, F, F) {
    let t11257 = t3521 * t4611;
    let t11259 = t1876 * t1814;
    let t11260 = t1636 * t4658;
    let t11261 = t11259 * t11260;
    let t11264 = t4644 * t1824;
    let t11265 = t4609 * t11264;
    let t11269 = t1797 * t1646 * t708;
    (t11257, t11261, t11265, t11269)
}
