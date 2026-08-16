//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1243/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1243<F: Float>(t10466: F, t283: F, t990: F, t3049: F, t982: F, t26748: F, t26757: F, t14443: F, t26766: F, t7703: F, t14447: F, t26696: F) -> (F, F, F, F, F) {
    let t93366 = t10466 * t283 * t990;
    let t93394 = t3049 * t982 * t990;
    let t93403 = t26748 * t26757;
    let t93406 = t7703 * t14443 * t26766;
    let t93409 = t7703 * t14447 * t26696;
    (t93366, t93394, t93403, t93406, t93409)
}
