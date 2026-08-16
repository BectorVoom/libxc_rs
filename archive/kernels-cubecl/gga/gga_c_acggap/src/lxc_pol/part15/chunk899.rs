//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 899/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk899<F: Float>(t1998: F, t3761: F, t141: F, t167: F, t2035: F, t1089: F, t2079: F, t429: F, t7542: F, t7457: F, t7458: F, t7459: F) -> (F, F, F, F, F) {
    let t30777 = t1998 * t3761;
    let t30779 = t167 * t141;
    let t30780 = t2035 * t30779;
    let t30786 = t2079 * t1089 * t429 * t7542;
    let t30790 = t7457 * t7458 * t429 * t7459;
    (t30777, t30779, t30780, t30786, t30790)
}
