//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 628/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk628<F: Float>(t3863: F, t521: F, t1320: F, t1333: F, t123: F, t520: F) -> (F, F, F) {
    let t3865 = F::cast_from(32.0_f64) * t3863 * t521;
    let t3867 = F::cast_from(8.0_f64) * t1320 * t1333;
    let t3869 = t520 * t123;
    (t3865, t3867, t3869)
}
