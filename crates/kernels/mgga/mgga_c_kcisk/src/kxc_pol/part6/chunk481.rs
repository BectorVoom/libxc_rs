//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 481/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk481<F: Float>(t4597: F, t708: F, t1797: F, t574: F, t1876: F, t682: F, t1849: F, t1646: F, t673: F, t298: F, t446: F, t569: F) -> (F, F, F, F, F, F, F) {
    let t4598 = t708 * t4597;
    let t4603 = t1797 * t574;
    let t4604 = t4603 * t708;
    let t4609 = t1876 * t682;
    let t4614 = t708 * t1849;
    let t4623 = t1646 * t708;
    let t4629 = t673 * t574;
    let t4636 = t298 * t446 * t569;
    (t4598, t4604, t4609, t4614, t4623, t4629, t4636)
}
