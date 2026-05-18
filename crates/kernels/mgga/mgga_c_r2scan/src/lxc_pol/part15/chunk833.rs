//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 833/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk833<F: Float>(t2124: F, t495: F, t7503: F, t6217: F, t7460: F, t1593: F, t2562: F, t360: F, t6359: F, t920: F, t1553: F, t6363: F) -> (F, F, F, F, F, F) {
    let t7509 = t2124 * t7503 * t495;
    let t7512 = t6217 * t7460;
    let t7513 = t2562 * t1593;
    let t7514 = t360 * t7513;
    let t7517 = t6359 * t920;
    let t7518 = t6363 * t1553;
    (t7509, t7512, t7513, t7514, t7517, t7518)
}
