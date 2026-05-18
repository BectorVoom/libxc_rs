//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1193/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1193<F: Float>(t1711: F, t7086: F, t125961: F, t27799: F, t27363: F, t33: F, t126017: F, t196: F, t197: F, t28230: F, t13426: F, t8461: F) -> (F, F, F, F, F, F) {
    let t127212 = t1711 * t7086;
    let t127218 = t27799 * t125961;
    let t127227 = t33 * t27363;
    let t127284 = t27799 * t126017;
    let t127317 = t28230 * t196 * t197;
    let t127365 = t13426 * t8461;
    (t127212, t127218, t127227, t127284, t127317, t127365)
}
