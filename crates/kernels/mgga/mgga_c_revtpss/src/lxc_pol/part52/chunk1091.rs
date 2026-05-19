//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1091/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1091<F: Float>(t5: F, t32584: F, t32590: F, t32599: F, t33621: F, t34169: F, t34173: F, t34177: F, t34181: F, t8620: F, t8623: F, t117: F, t7935: F, t8698: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t34187 = piecewise3::<F>(t8, F::new(0.0), -F::new(5.0) / F::new(72.0) * t34169 * t8623 + F::new(5.0) / F::new(12.0) * t32584 * t34173 + F::new(5.0) / F::new(18.0) * t32590 * t34177 + t32599 - F::new(5.0) / F::new(36.0) * t8620 * t34181 - F::new(5.0) / F::new(72.0) * t8620 * t33621);
    let t34188 = t34187 * t117;
    let t34191 = t8698 * t7935;
    (t34187, t34188, t34191)
}
