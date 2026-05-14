//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 775/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk775<F: Float>(t10739: F, t28371: F, t28375: F, t28383: F, t28391: F, t28410: F, t28412: F, t28415: F, t28417: F, t28420: F, t28423: F, t28426: F, t28431: F, t28435: F, t28408: F, t1664: F) -> (F,) {
    let t28437 = -0.82156666666666666668e-1 * t28410 - 0.28483875e1 * t28412 - t10739 - 0.76790625e-1 * t28415 + 0.142419375e1 * t28417 - 0.36514074074074074075e-1 * t28420 - 0.82156666666666666667e-1 * t28423 - 0.49293999999999999999e0 * t28426 + 0.11958666666666666667e1 * t28375 - 0.17938e1 * t28383 + 0.16431333333333333333e0 * t28431 - 0.33218518518518518518e0 * t28371 - 0.29896666666666666667e0 * t28391 + 0.3071625e0 * t28435;
    let t28438 = t28408 + t28437;
    let t28439 = t28438 * t1664;
    (t28439,)
}
