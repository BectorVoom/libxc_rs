//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 977/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk977<F: Float>(t10810: F, t2608: F, t574: F, t10698: F, t3588: F, t1010: F, t11033: F, t11036: F, t2381: F, t2391: F, t3358: F, t1070: F, t8355: F) -> (F, F, F, F, F, F, F) {
    let t11842 = t10810 * t2608;
    let t11843 = t574 * t11842;
    let t11845 = t10698 * t3588;
    let t11866 = t11033 * t1010;
    let t11868 = t11036 * t2381;
    let t11870 = t3358 * t2391;
    let t11872 = t8355 * t1070;
    (t11842, t11843, t11845, t11866, t11868, t11870, t11872)
}
