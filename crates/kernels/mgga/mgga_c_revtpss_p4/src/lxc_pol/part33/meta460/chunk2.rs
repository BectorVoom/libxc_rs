//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1671/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1671<F: Float>(t21332: F, t459: F, t225: F, t480: F, t12832: F, t17401: F, t17736: F, t17767: F, t17771: F, t17791: F, t17792: F, t21300: F, t21306: F, t21310: F, t21313: F, t21316: F, t3718: F, t484: F, t5335: F, t5348: F, t6690: F) -> (F, F, F) {
    let t21333 = t21332 * t459;
    let t21334 = t21333 * t225;
    let t21335 = t21334 * t480;
    let t21338 = -F::cast_from(0.42874018118069736972e-3_f64) * t17401 * t5348 - F::cast_from(0.21437009059034868486e-3_f64) * t3718 * t21300 - F::cast_from(0.42874018118069736972e-3_f64) * t12832 * t6690 - t17767 - t17771 - t17791 + t17792 / F::new(81.0) - F::cast_from(0.42874018118069736972e-3_f64) * t21306 * t5335 - F::cast_from(0.57165357490759649296e-3_f64) * t17736 * t21310 + F::cast_from(0.72409452821628889107e-2_f64) * t21313 * t484 - F::cast_from(0.22866142996303859718e-2_f64) * t21316 * t484 + F::cast_from(0.21437009059034868486e-3_f64) * t21335 * t484;
    (t21333, t21334, t21338)
}
