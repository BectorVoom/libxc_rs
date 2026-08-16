//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2118/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2118<F: Float>(t1873: F, t96356: F, t28002: F, t6534: F, t12725: F, t7467: F, t75560: F, t19451: F, t96654: F, t96655: F, t96659: F, t96661: F, t96663: F, t96665: F, t96667: F, t96669: F, t96671: F) -> F {
    let t96673 = F::cast_from(4.0_f64) * t96356 * t1873;
    let t96675 = F::cast_from(4.0_f64) * t28002 * t6534;
    let t96677 = F::cast_from(4.0_f64) * t12725 * t7467;
    let t96679 = F::cast_from(2.0_f64) * t75560 * t1873;
    let t96681 = F::cast_from(2.0_f64) * t19451 * t6534;
    let t96682 = t96654 + F::cast_from(2.0_f64) * t96655 + t96659 + t96661 + t96663 + t96665 + t96667 + t96669 + t96671 + t96673 + t96675 + t96677 + t96679 + t96681;
    t96682
}
