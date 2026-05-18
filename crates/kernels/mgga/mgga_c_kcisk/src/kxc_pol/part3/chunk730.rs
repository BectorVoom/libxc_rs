//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 730/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk730<F: Float>(t10441: F, t1876: F, t4598: F, t10487: F, t708: F, t4595: F, t1648: F, t4652: F, t7028: F, t1417: F, t4686: F, t4626: F) -> (F, F, F, F, F, F) {
    let t11325 = t1876 * t4598 * t10441;
    let t11328 = t708 * t10487;
    let t11330 = t4595 * t11328 * t10441;
    let t11334 = t708 * t1648 * t4652;
    let t11335 = t7028 * t11334;
    let t11338 = t1417 * t4686;
    let t11340 = t1417 * t4626;
    (t11325, t11330, t11334, t11335, t11338, t11340)
}
