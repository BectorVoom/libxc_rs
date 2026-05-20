//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2396/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2396<F: Float>(t225: F, t42066: F, t41306: F, t367: F, t371: F, t373: F, t9291: F, t11773: F, t11865: F, t42051: F, t366: F, t1025: F, t1026: F, t2434: F) -> (F, F, F, F, F, F, F) {
    let t42067 = t225 * t42066;
    let t42078 = F::cast_from(0.15365432098765432099e0_f64) * t41306;
    let t42121 = F::cast_from(0.14820648238345094262e-3_f64) * t367 * t371 * t9291 * t373;
    let t42155 = t11865 * t11773;
    let t42261 = t42051 * t225;
    let t42262 = t42261 * t366;
    let t42274 = t1025 * t371 * t2434 * t1026;
    (t42067, t42078, t42121, t42155, t42261, t42262, t42274)
}
