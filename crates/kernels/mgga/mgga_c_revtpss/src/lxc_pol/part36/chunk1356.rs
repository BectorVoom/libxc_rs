//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1356/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1356<F: Float>(t104818: F, t112456: F, t112483: F, t112485: F, t112487: F, t112491: F, t1791: F, t24619: F, t24726: F, t24846: F, t24858: F, t29062: F, t29083: F, t6611: F, t6647: F, t6679: F, t6683: F, t7624: F, t97193: F, t97296: F) -> F {
    let t116258 = F::cast_from(0.68598428988911579154e-2_f64) * t29062 * t6647 - F::cast_from(0.25724410870841842183e-2_f64) * t97193 * t24619 - F::cast_from(0.13719685797782315831e-1_f64) * t104818 * t6611 - F::cast_from(0.43445671692977333464e-1_f64) * t112456 * t1791 + t97296 + F::cast_from(0.45732285992607719436e-2_f64) * t29083 * t6679 + F::cast_from(0.91464571985215438873e-2_f64) * t29083 * t6683 - F::cast_from(0.28582678745379824648e-3_f64) * t7624 * t24858 + F::cast_from(0.28582678745379824648e-2_f64) * t7624 * t24846 - F::cast_from(0.17149607247227894789e-2_f64) * t7624 * t24726 + F::cast_from(0.17149607247227894789e-2_f64) * t112483 + F::cast_from(0.91464571985215438873e-2_f64) * t112485 + F::cast_from(0.28963781128651555642e-1_f64) * t112487 - F::cast_from(0.91464571985215438873e-2_f64) * t112491;
    t116258
}
