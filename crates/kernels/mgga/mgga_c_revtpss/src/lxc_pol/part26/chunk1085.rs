//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1085/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1085<F: Float>(t26496: F, t786: F, t2467: F, t25431: F, t26482: F, t225: F, t26473: F, t2470: F, t7406: F, t7064: F, t2061: F, t2722: F) -> (F, F, F, F, F, F, F) {
    let t26497 = t786 * t26496;
    let t26498 = t26497 * t2467;
    let t26500 = t25431 * t26482;
    let t26502 = t26473 * t225;
    let t26506 = t7406 * t2470;
    let t26508 = F::cast_from(0.17135234354032049604e-1_f64) * t7064 * t26506;
    let t26509 = t2061 * t2722;
    (t26497, t26498, t26500, t26502, t26506, t26508, t26509)
}
