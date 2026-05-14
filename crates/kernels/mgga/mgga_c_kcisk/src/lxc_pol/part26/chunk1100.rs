//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1100/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1100<F: Float>(t2737: F, t32388: F, t9529: F, t9532: F, t9524: F, t4419: F, t9518: F) -> (F, F, F, F) {
    let t32390 = 0.11574074074074074074e-2 * t2737 * t32388;
    let t32391 = t9529 * t9532;
    let t32399 = t9524 * t9532;
    let t32401 = t4419 * t9518;
    (t32390, t32391, t32399, t32401)
}
