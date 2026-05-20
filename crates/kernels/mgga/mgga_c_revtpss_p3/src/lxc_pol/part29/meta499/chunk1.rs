//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1815/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1815<F: Float>(t531: F, t8107: F, t7238: F, t2014: F, t2056: F, t2093: F, t2108: F, t27123: F, t27126: F, t27833: F, t28167: F, t28760: F, t28927: F, t28929: F, t28932: F, t28935: F, t4248: F, t5787: F, t651: F, t7235: F, t7367: F, t7374: F, t7489: F, t7732: F, t7898: F, t8079: F, t8109: F) -> (F, F, F) {
    let t28938 = t531 * t8107;
    let t28939 = t28938 * t7238;
    let t28942 = t2014 * t28927 + F::new(3.0) * t2014 * t28932 + F::new(3.0) * t2014 * t28935 + F::new(3.0) * t2014 * t28939 - F::new(2.0) * t2056 * t27123 - F::new(2.0) * t2056 * t27126 + t2093 * t5787 + t2108 * t27833 + F::new(6.0) * t28167 * t28929 - F::new(2.0) * t28760 * t651 - F::new(2.0) * t4248 * t7374 + F::new(3.0) * t7235 * t8079 + t7235 * t8109 - F::new(2.0) * t7367 * t7732 + F::new(3.0) * t7489 * t7898;
    (t28938, t28939, t28942)
}
