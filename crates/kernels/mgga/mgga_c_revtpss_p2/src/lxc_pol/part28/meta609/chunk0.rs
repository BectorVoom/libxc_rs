//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2119/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2119<F: Float>(t13829: F, t2661: F, t94550: F, t1873: F, t94519: F, t94520: F, t94527: F, t94537: F, t94540: F, t26004: F, t5690: F, t94514: F, t94523: F, t94526: F, t94530: F, t94534: F) -> F {
    let t98258 = t2661 * t94550 * t13829;
    let t98259 = F::cast_from(0.57165357490759649296e-4_f64) * t98258;
    let t98260 = t94519 * t1873;
    let t98263 = F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t94520;
    let t98264 = F::cast_from(0.1219527626469539185e-2_f64) * t94527;
    let t98267 = F::cast_from(0.10164000561857065645e-4_f64) * t94537;
    let t98268 = F::cast_from(0.72286371995927450868e-4_f64) * t94540;
    let t98269 = t26004 * t5690;
    let t98270 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t98269;
    let t98271 = -t98259 - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t98260 - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t94514 - t98263 - t94523 + t94526 - t98264 + F::cast_from(0.57165357490759649296e-4_f64) * t94530 - F::cast_from(0.28582678745379824648e-3_f64) * t94534 + t98267 - t98268 + t98270;
    t98271
}
