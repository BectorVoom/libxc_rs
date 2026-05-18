//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1246/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1246<F: Float>(t5: F, t128422: F, t128474: F, t117: F, t125385: F, t125387: F, t125389: F, t125391: F, t128367: F, t32176: F, t32178: F, t33644: F, t33646: F, t8564: F) -> (F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t128476 = piecewise3::<f64>(t8, F::new(0.0), t128422 + t128474);
    let t128477 = t128476 * t117;
    let t128478 = t128367 + t33644 + t33646 + t128477 + t8564 + t32176 + t32178 + t125385 + t125387 + t125389 + t125391;
    (t128477, t128478)
}
