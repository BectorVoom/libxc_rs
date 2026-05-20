//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2180/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2180<F: Float>(t6416: F, t775: F, t106501: F, t27799: F, t25759: F, t77441: F, t1711: F, t4537: F, t106539: F, t1113: F, t1940: F, t1963: F, t2403: F, t25206: F, t25440: F, t27364: F, t27773: F, t27777: F, t29705: F, t29940: F, t29946: F, t29967: F, t50080: F, t7091: F, t7200: F, t7783: F, t7862: F, t7869: F, t92819: F, t99555: F) -> F {
    let t107970 = t6416 * t775;
    let t107974 = t27799 * t106501;
    let t107985 = t25759 * t77441;
    let t107988 = t1711 * t4537;
    let t108001 = F::new(3.0) * t50080 * t29940 + F::new(3.0) / F::new(2.0) * t2403 * t29705 * t7200 + F::new(3.0) / F::new(2.0) * t2403 * t1963 * t107970 + F::new(6.0) * t25206 * t107974 + F::new(3.0) * t2403 * t7783 * t27773 - t1940 * t99555 * t7869 - t106539 + F::new(3.0) * t2403 * t27364 * t7862 - F::new(3.0) * t25206 * t107985 - t1940 * t7091 * t107988 - F::new(3.0) * t92819 * t29946 + t1940 * t29705 * t1113 / F::new(2.0) - t1940 * t25440 * t29967 + F::new(3.0) * t2403 * t7783 * t27777;
    t108001
}
