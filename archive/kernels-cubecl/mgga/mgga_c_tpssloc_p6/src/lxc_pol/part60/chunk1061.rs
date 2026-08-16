//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1061/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1061<F: Float>(t127560: F, t127562: F, t128908: F, t128909: F, t128922: F, t128924: F, t128926: F, t129008: F, t130442: F, t1774: F, t2040: F, t28002: F, t32350: F, t33690: F, t34137: F, t34150: F, t34170: F, t4028: F, t510: F, t5494: F, t6287: F, t6468: F, t7787: F, t7796: F, t8103: F, t8829: F, t8835: F, t8840: F) -> F {
    let t130492 = -F::cast_from(2.0_f64) * t129008 * t2040 - t130442 * t510 - F::cast_from(2.0_f64) * t1774 * t34137 - F::cast_from(4.0_f64) * t28002 * t8835 - F::cast_from(2.0_f64) * t32350 * t5494 - F::cast_from(4.0_f64) * t33690 * t7796 - F::cast_from(4.0_f64) * t34150 * t4028 - F::cast_from(4.0_f64) * t34170 * t4028 - t6287 * t8829 + t6468 * t8840 - F::cast_from(2.0_f64) * t7787 * t8103 - t127560 - t127562 + t128908 + t128909 - t128922 - t128924 + t128926;
    t130492
}
