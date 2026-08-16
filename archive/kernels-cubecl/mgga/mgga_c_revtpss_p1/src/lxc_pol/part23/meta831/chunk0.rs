//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2691/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2691<F: Float>(t1063: F, t11986: F, t247: F, t6096: F, t20112: F, t359: F, t19572: F, t3302: F, t12046: F, t1678: F, t342: F, t1086: F, t6343: F, t994: F) -> (F, F, F, F, F) {
    let t67575 = t1063 * t247 * t11986 * t6096;
    let t67595 = t359 * t20112;
    let t67599 = t19572 * t3302;
    let t67644 = t342 * t12046 * t1678;
    let t67652 = t994 * t1086 * t6343;
    (t67575, t67595, t67599, t67644, t67652)
}
