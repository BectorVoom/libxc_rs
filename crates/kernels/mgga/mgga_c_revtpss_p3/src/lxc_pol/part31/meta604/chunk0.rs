//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2039/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2039<F: Float>(t13730: F, t2023: F, t2782: F, t10073: F, t25938: F, t27836: F, t14079: F, t26054: F, t7289: F, t97925: F, t2470: F, t27872: F) -> (F, F, F, F, F) {
    let t98001 = F::cast_from(0.21951497276451705328e-1_f64) * t2782 * t2023 * t13730;
    let t98003 = t10073 * t27836 * t25938;
    let t98010 = F::cast_from(0.19514881078765566038e-1_f64) * t26054 * t14079;
    let t98011 = t7289 * t97925;
    let t98028 = t27872 * t2470;
    (t98001, t98003, t98010, t98011, t98028)
}
