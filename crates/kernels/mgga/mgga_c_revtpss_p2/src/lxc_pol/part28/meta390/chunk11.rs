//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1478/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1478<F: Float>(t13750: F, t14088: F, t14279: F, t14302: F, t1343: F, t13664: F, t13667: F, t13669: F, t13671: F, t13673: F, t13674: F, t13682: F, t13683: F, t13716: F, t13885: F, t13886: F, t13888: F, t1450: F, t198: F, t3889: F, t4135: F, t4139: F, t4144: F, t532: F, t5532: F, t5541: F, t5542: F, t9524: F, t9542: F, t9854: F, t9865: F, t9868: F) -> (F, F) {
    let t14304 = t13750 + t14088 + t14279 + t14302;
    let t14308 = t14304 * t1450 * t198 * t532 + F::new(3.0) * t1343 * t13716 * t198 + F::new(2.0) * t13674 * t4144 * t5541 + F::new(3.0) * t3889 * t4139 * t5532 - t4135 * t5541 * t5542 - t13664 + t13667 + t13669 - t13671 + t13673 + t13682 + t13683 - t13885 + t13886 + t13888 - t9524 + t9542 - t9854 + t9865 + t9868;
    (t14304, t14308)
}
