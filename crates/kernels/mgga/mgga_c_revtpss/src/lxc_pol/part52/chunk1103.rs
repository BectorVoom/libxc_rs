//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1103/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1103<F: Float>(t2107: F, t33651: F, t2014: F, t2056: F, t33602: F, t34279: F, t34285: F, t34290: F, t34294: F, t34300: F, t34304: F, t34326: F, t4248: F, t569: F, t651: F, t6985: F, t7359: F, t7732: F, t7746: F, t7978: F, t7988: F, t8637: F) -> (F, F) {
    let t34328 = t2107 * t33651;
    let t34329 = t2014 * t34328;
    let t34330 = -F::new(2.0) * t2056 * t33602 - F::new(2.0) * t34279 * t651 - F::new(2.0) * t34290 * t651 + t34326 * t569 - F::new(2.0) * t4248 * t8637 - F::new(2.0) * t6985 * t7978 - F::new(2.0) * t6985 * t7988 - F::new(2.0) * t7359 * t7746 - F::new(2.0) * t7732 * t8637 - t34285 - t34294 + t34300 - t34304 - t34329;
    (t34328, t34330)
}
