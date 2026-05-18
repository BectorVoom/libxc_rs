//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1171/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1171<F: Float>(t5: F, t131276: F, t131318: F, t117: F, t125510: F, t125512: F, t125514: F, t125521: F, t125525: F, t129326: F, t129328: F, t129332: F, t129335: F, t129339: F, t129342: F, t129344: F, t1310: F, t131234: F, t1911: F, t33343: F, t33381: F, t34874: F, t35014: F, t4248: F, t508: F, t649: F, t651: F, t670: F, t671: F) -> (F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t131320 = piecewise3::<f64>(t8, F::new(0.0), t131276 + t131318);
    let t131321 = t131320 * t117;
    let t131331 = -F::new(2.0) * t35014 * t651 * t670 - t1310 * t34874 - F::new(2.0) * t131234 * t671 - t131321 * t508 + t1911 * t33381 - F::new(2.0) * t33343 * t4248 - t35014 * t649 + t125510 + t125512 - t125514 - t125521 - t125525 - F::new(4.0) * t129326 - F::new(4.0) * t129328 - F::new(4.0) * t129332 - F::new(4.0) * t129335 - F::new(2.0) * t129339 + F::new(6.0) * t129342 + F::new(6.0) * t129344;
    (t131321, t131331)
}
