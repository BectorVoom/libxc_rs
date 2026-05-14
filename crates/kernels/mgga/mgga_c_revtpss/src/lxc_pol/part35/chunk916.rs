//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 916/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk916<F: Float>(t1222: F, t1261: F, t12809: F, t12855: F, t1808: F, t21242: F, t24817: F, t24821: F, t24827: F, t24831: F, t24836: F, t24840: F, t24846: F, t24858: F, t5373: F, t5381: F, t5391: F, t6653: F, t6673: F, t6679: F, t6683: F) -> (F,) {
    let t24861 = -t1222 * t24817 / 288.0 - t1222 * t24821 / 48.0 - t5373 * t6653 / 27.0 - 7.0 / 648.0 * t1222 * t24827 + t1222 * t24831 / 36.0 - 0.12862205435420921092e-2 * t12855 * t24836 + 0.64311027177104605458e-3 * t12809 * t24840 + 0.7145669686344956162e-3 * t5381 * t6673 + 0.14291339372689912324e-2 * t1261 * t24846 + 0.45732285992607719436e-2 * t21242 * t1808 + 0.22866142996303859718e-2 * t5391 * t6679 + 0.45732285992607719436e-2 * t5391 * t6683 - 0.42874018118069736972e-3 * t5381 * t6679 - 0.14291339372689912324e-3 * t1261 * t24858;
    (t24861,)
}
