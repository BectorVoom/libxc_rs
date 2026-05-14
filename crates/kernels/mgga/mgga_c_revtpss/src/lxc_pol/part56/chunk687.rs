//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 687/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk687<F: Float>(t2033: F, t4147: F, t2121: F, t8435: F, t2247: F, t1937: F, t7586: F, t1936: F, t2163: F) -> (F, F, F, F, F) {
    let t8717 = t4147 * t2033;
    let t8736 = t8435 * t2121;
    let t8737 = t2247 * t8736;
    let t8743 = t7586 * t1937;
    let t8749 = t2163 * t1936;
    (t8717, t8736, t8737, t8743, t8749)
}
