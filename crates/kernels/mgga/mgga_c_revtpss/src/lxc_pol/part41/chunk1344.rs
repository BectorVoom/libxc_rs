//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1344/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1344<F: Float>(t100: F, t1513: F, t2339: F, t69: F, t96: F, t31027: F, t31268: F, t10199: F, t116: F, t31292: F, t1913: F, t8302: F, t2192: F, t5789: F, t2184: F, t5808: F) -> (F, F, F, F, F, F, F, F) {
    let t117500 = t100 * t1513;
    let t117505 = t69 * t2339 * t96;
    let t117510 = 20.0 / 9.0 * t31027 * t31268;
    let t117544 = t10199 * t2339;
    let t117758 = t116 * t31292;
    let t117772 = 2.0 * t1913 * t8302;
    let t117774 = 2.0 * t5789 * t2192;
    let t117781 = 2.0 * t2184 * t5808;
    (t117500, t117505, t117510, t117544, t117758, t117772, t117774, t117781)
}
