//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1050/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1050<F: Float>(t3863: F, t521: F, t1320: F, t1333: F, t198: F, t2522: F, t2562: F, t2569: F, t2579: F, t2587: F, t3827: F, t3828: F, t3829: F, t3852: F, t3854: F, t3856: F, t3859: F, t3862: F) -> (F, F, F) {
    let t3865 = F::new(32.0) * t3863 * t521;
    let t3867 = F::new(8.0) * t1320 * t1333;
    let t3868 = F::new(6.0) * t198 * t3828 * t3829 - t2522 - t2562 - t2569 + t2579 + t2587 - t3827 + t3852 + t3854 + t3856 + t3859 + t3862 - t3865 - t3867;
    (t3865, t3867, t3868)
}
