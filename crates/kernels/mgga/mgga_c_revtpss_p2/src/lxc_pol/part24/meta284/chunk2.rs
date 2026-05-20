//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1064/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1064<F: Float>(t1647: F, t4980: F, t359: F, t6343: F, t1086: F, t6235: F, t4995: F, t3153: F, t6299: F, t1678: F, t3298: F, t342: F) -> (F, F, F, F, F, F, F) {
    let t19526 = t1647 * t4980;
    let t19556 = t359 * t6343;
    let t19566 = t6235 * t1086;
    let t19569 = t1647 * t4995;
    let t19572 = t6299 * t3153;
    let t19602 = t3298 * t1678;
    let t19603 = t342 * t19602;
    (t19526, t19556, t19566, t19569, t19572, t19602, t19603)
}
