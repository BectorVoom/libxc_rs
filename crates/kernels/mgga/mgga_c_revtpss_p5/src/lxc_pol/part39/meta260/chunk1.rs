//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 972/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk972<F: Float>(t3: F, t8372: F, t1518: F, t8295: F, t117: F, t8362: F, t1916: F, t1918: F, t2187: F, t2189: F, t572: F, t573: F, param_d: F) -> (F, F, F, F, F) {
    let t8373 = t3 * t8372;
    let t8377 = param_d * t8372;
    let t8383 = t8295 * t1518;
    let t8386 = t117 * t8362;
    let t8389 = F::new(3.0) * t1916 * t2189 + F::new(3.0) * t1918 * t2187 + F::new(6.0) * t572 * t8383 + F::new(3.0) * t572 * t8386 + t573 * t8377;
    (t8373, t8377, t8383, t8386, t8389)
}
