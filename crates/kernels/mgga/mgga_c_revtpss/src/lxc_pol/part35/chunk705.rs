//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 705/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk705<F: Float>(t3: F, t8113: F, t1518: F, t7553: F, t117: F, t7983: F, t1916: F, t1918: F, t2113: F, t2115: F, t572: F, t573: F, param_d: F) -> (F, F, F, F, F) {
    let t8114 = t3 * t8113;
    let t8118 = param_d * t8113;
    let t8124 = t7553 * t1518;
    let t8127 = t117 * t7983;
    let t8130 = F::new(3.0) * t1916 * t2115 + F::new(3.0) * t1918 * t2113 + F::new(6.0) * t572 * t8124 + F::new(3.0) * t572 * t8127 + t573 * t8118;
    (t8114, t8118, t8124, t8127, t8130)
}
