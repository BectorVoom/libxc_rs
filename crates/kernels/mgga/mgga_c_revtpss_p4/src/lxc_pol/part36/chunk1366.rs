//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1366/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1366<F: Float>(t265: F, t502: F, t105669: F, t112958: F, t114089: F, t116331: F, t116381: F, t116430: F, t116469: F, t116520: F, t116565: F, t116607: F, t116649: F, t1300: F, t1832: F, t198: F, t24501: F, t25026: F, t27041: F, t29317: F, t336: F, t5023: F, t6748: F, t6752: F, t7673: F, t97498: F) -> F {
    let t503 = t265 < t502;
    let t116675 = piecewise3::<F>(t503, t198 * t336 * (t116331 + t116381 + t116430 + t116469 + t116520 + t116565 + t116607 + t116649) * t1300 - F::cast_from(3.0_f64) * t5023 * t112958 * t1832 + F::cast_from(6.0_f64) * t5023 * t105669 * t6752 - F::cast_from(3.0_f64) * t5023 * t29317 * t6748 - F::cast_from(6.0_f64) * t5023 * t97498 * t24501 + F::cast_from(6.0_f64) * t5023 * t27041 * t1832 * t6748 - t5023 * t7673 * t25026, t114089);
    t116675
}
