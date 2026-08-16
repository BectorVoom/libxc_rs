//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2115/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2115<F: Float>(t13967: F, t26028: F, t13937: F, t13981: F, t94479: F, t2689: F, t27936: F, t13857: F, t94564: F, t25978: F, t5629: F, t1885: F, t94459: F) -> (F, F, F, F, F, F, F, F) {
    let t98211 = t26028 * t13967;
    let t98213 = t26028 * t13937;
    let t98215 = t26028 * t13981;
    let t98217 = F::cast_from(0.4065600224742826258e-4_f64) * t94479;
    let t98218 = t2689 * t27936;
    let t98220 = t94564 * t13857;
    let t98222 = t25978 * t5629;
    let t98224 = t94459 * t1885;
    (t98211, t98213, t98215, t98217, t98218, t98220, t98222, t98224)
}
