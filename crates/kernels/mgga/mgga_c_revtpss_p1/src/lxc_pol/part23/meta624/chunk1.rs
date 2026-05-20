//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2311/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2311<F: Float>(t24834: F, t3604: F, t3720: F, t3611: F, t24232: F, t247: F, t3618: F, t1264: F, t24248: F, t1222: F, t1261: F, t12809: F, t12855: F, t1808: F, t21242: F, t24817: F, t24821: F, t24827: F, t24831: F, t5373: F, t5381: F, t5391: F, t6653: F, t6673: F, t6679: F, t6683: F) -> (F, F, F, F, F, F, F) {
    let t24835 = t24834 * t3604;
    let t24836 = t3720 * t24835;
    let t24839 = t24834 * t3611;
    let t24840 = t3720 * t24839;
    let t24846 = t247 * t3618 * t24232;
    let t24858 = t247 * t1264 * t24248;
    let t24861 = -t1222 * t24817 / F::new(288.0) - t1222 * t24821 / F::new(48.0) - t5373 * t6653 / F::new(27.0) - F::new(7.0) / F::new(648.0) * t1222 * t24827 + t1222 * t24831 / F::new(36.0) - F::cast_from(0.12862205435420921092e-2_f64) * t12855 * t24836 + F::cast_from(0.64311027177104605458e-3_f64) * t12809 * t24840 + F::cast_from(0.7145669686344956162e-3_f64) * t5381 * t6673 + F::cast_from(0.14291339372689912324e-2_f64) * t1261 * t24846 + F::cast_from(0.45732285992607719436e-2_f64) * t21242 * t1808 + F::cast_from(0.22866142996303859718e-2_f64) * t5391 * t6679 + F::cast_from(0.45732285992607719436e-2_f64) * t5391 * t6683 - F::cast_from(0.42874018118069736972e-3_f64) * t5381 * t6679 - F::cast_from(0.14291339372689912324e-3_f64) * t1261 * t24858;
    (t24835, t24836, t24839, t24840, t24846, t24858, t24861)
}
