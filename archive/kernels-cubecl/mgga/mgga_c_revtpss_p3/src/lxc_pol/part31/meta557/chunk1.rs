//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1967/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1967<F: Float>(t30188: F, t572: F, t5920: F, t7330: F, t117: F, t30004: F, t1918: F, t2040: F, t30171: F, t30180: F, t30182: F, t30184: F, t30187: F, t573: F, t6945: F, t6948: F, t7944: F) -> (F, F, F) {
    let t30190 = F::cast_from(12.0_f64) * t572 * t30188;
    let t30191 = t7330 * t5920;
    let t30193 = F::cast_from(6.0_f64) * t572 * t30191;
    let t30194 = t117 * t30004;
    let t30196 = F::cast_from(3.0_f64) * t572 * t30194;
    let t30197 = F::cast_from(6.0_f64) * t1918 * t7944 + F::cast_from(6.0_f64) * t2040 * t6945 + F::cast_from(3.0_f64) * t2040 * t6948 + t30171 * t573 + t30180 + t30182 + t30184 + t30187 + t30190 + t30193 + t30196;
    (t30191, t30194, t30197)
}
