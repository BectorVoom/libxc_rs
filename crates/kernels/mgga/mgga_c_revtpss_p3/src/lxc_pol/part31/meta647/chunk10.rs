//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2134/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2134<F: Float>(t106481: F, t892: F, t198: F, t205: F, t7782: F, t25207: F, t77441: F, t1544: F, t580: F, t98646: F, t25206: F, t105898: F, t105902: F, t105906: F, t105909: F, t105919: F, t105924: F, t105930: F, t1940: F, t1963: F, t2403: F, t25440: F, t27160: F, t29591: F, t29602: F, t29716: F, t30: F, t4541: F, t7087: F) -> (F, F, F, F) {
    let t106482 = t106481 * t892;
    let t106487 = t198 * t205 * t7782;
    let t106490 = t25207 * t77441;
    let t106494 = t98646 * t580 * t1544;
    let t106496 = F::new(6.0) * t25206 * t106494;
    let t106497 = F::new(3.0) / F::new(2.0) * t2403 * t1963 * t105898 + F::new(3.0) * t4541 * t1963 * t105902 - F::new(3.0) * t25206 * t105906 + F::new(3.0) * t2403 * t1963 * t105909 + F::new(3.0) * t4541 * t7087 * t29591 + F::new(3.0) * t2403 * t7087 * t29602 + F::new(3.0) / F::new(2.0) * t2403 * t1963 * t105919 - F::new(3.0) / F::new(2.0) * t25206 * t105924 - t105930 - t1940 * t25440 * t29716 + t1940 * t106482 * t30 / F::new(2.0) + F::new(6.0) * t106487 * t27160 - F::new(3.0) * t25206 * t106490 + t106496;
    (t106482, t106487, t106496, t106497)
}
