//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1247/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1247<F: Float>(t21233: F, t21236: F, t21239: F, t21251: F, t21255: F, t21257: F, t21320: F, t21324: F, t21329: F, t21331: F, t21333: F, t21186: F, t21196: F, t21217: F, t21220: F, t21223: F, t21225: F, t21308: F, t21313: F, t21315: F, t21318: F, t21814: F, t21815: F, t21817: F) -> F {
    let t21819 = -t21320 + t21233 + t21236 + t21239 - t21324 - t21329 - t21331 - t21333 - t21251 + t21255 + t21257;
    let t21822 = t21814 + t21815 + t21817 - t21308 + t21313 - t21315 - t21318 + t21186 - t21196 + t21217 + t21220 + t21223 + t21225 + t21819;
    t21822
}
