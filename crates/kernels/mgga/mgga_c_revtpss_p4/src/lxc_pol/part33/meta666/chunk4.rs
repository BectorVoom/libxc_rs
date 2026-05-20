//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2183/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2183<F: Float>(t22115: F, t26028: F, t2018: F, t22125: F, t807: F, t102515: F, t102526: F, t102527: F, t94472: F, t94474: F, t94477: F, t94479: F, t98194: F, t98203: F, t98207: F) -> F {
    let t108583 = t26028 * t22115;
    let t108587 = t807 * t2018 * t22125;
    let t108589 = -t98194 - t94472 + t102515 + t94474 + t98203 - F::cast_from(0.42874018118069736972e-3_f64) * t108583 + t98207 - t94477 + F::cast_from(0.2032800112371413129e-4_f64) * t94479 - t102526 + F::cast_from(0.57165357490759649296e-4_f64) * t108587 - t102527;
    t108589
}
