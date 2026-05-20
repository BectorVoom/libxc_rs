//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1104/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1104<F: Float>(t30555: F, t30625: F, t3: F, t2055: F, t5883: F, t1518: F, t28986: F, t5920: F, t7553: F, t117: F, t30570: F, t1916: F, t1918: F, t2113: F, t2115: F, t572: F, t573: F, t6941: F, t6945: F, t6948: F, t8118: F, t8124: F, t8127: F, param_d: F) -> (F, F, F, F, F, F, F, F) {
    let t30626 = t30555 + t30625;
    let t30627 = t3 * t30626;
    let t30637 = param_d * t30626;
    let t30651 = t5883 * t2055;
    let t30654 = t28986 * t1518;
    let t30657 = t7553 * t5920;
    let t30660 = t117 * t30570;
    let t30663 = F::new(12.0) * t1916 * t8124 + F::new(6.0) * t1916 * t8127 + F::new(6.0) * t1918 * t8118 + F::new(6.0) * t2113 * t6945 + F::new(3.0) * t2113 * t6948 + F::new(3.0) * t2115 * t6941 + t30637 * t573 + F::new(6.0) * t30651 * t572 + F::new(12.0) * t30654 * t572 + F::new(6.0) * t30657 * t572 + F::new(3.0) * t30660 * t572;
    (t30626, t30627, t30637, t30651, t30654, t30657, t30660, t30663)
}
