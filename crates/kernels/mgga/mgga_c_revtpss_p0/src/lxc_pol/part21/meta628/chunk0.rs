//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2392/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2392<F: Float>(t2237: F, t2482: F, t849: F, t2677: F, t10489: F, t221: F, t2674: F, t2675: F, t234: F, t9801: F, t10887: F, t136: F, t2475: F) -> (F, F, F, F, F, F) {
    let t40710 = t2482 * t849 * t2237;
    let t40711 = t40710 * t2677;
    let t40719 = t2674 * t2675 * t221 * t10489;
    let t40721 = t9801 * t234;
    let t40722 = t40721 * t10887;
    let t40724 = t2475 * t136;
    (t40710, t40711, t40719, t40721, t40722, t40724)
}
