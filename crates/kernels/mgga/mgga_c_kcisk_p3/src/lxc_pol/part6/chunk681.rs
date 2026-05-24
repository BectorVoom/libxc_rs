//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 681/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk681<F: Float>(t1814: F, t1876: F, t1646: F, t1797: F, t708: F, t4594: F, t574: F, t4595: F, t682: F, t139: F, t3516: F, t41: F) -> (F, F, F, F, F) {
    let t11259 = t1876 * t1814;
    let t11269 = t1797 * t1646 * t708;
    let t11279 = t4594 * t574 * t708;
    let t11285 = t4595 * t682;
    let t11313 = t139 * t3516 * t41;
    (t11259, t11269, t11279, t11285, t11313)
}
