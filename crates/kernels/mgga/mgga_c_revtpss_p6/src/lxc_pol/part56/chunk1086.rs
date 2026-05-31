//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1086/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1086<F: Float>(t1469: F, t8442: F, t33624: F, t644: F, t8621: F, t1497: F, t36: F, t606: F, t125209: F, t34258: F, t7002: F, t32392: F, t7741: F) -> (F, F, F, F, F, F) {
    let t125314 = t8442 * t1469;
    let t125328 = t8621 * t33624 * t644;
    let t125335 = t1497 * t36;
    let t125336 = t125335 * t606;
    let t125337 = t8442 * t125336;
    let t125344 = F::cast_from(2.0_f64) * t125209;
    let t125377 = F::cast_from(4.0_f64) * t34258 * t7002;
    let t125379 = F::cast_from(4.0_f64) * t32392 * t7741;
    (t125314, t125328, t125337, t125344, t125377, t125379)
}
