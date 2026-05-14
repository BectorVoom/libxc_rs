//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 782/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk782<F: Float>(t24668: F, t2579: F, t14127: F, t6163: F, t8392: F, t6162: F, t9787: F, t6154: F, t729: F, t1456: F, t2413: F, t724: F, t2405: F, t2594: F, t1882: F, t6101: F) -> (F, F, F, F, F, F, F, F) {
    let t24669 = t24668 * t2579;
    let t24670 = t14127 * t24669;
    let t24673 = t8392 * t6163;
    let t24675 = t9787 * t6162;
    let t24679 = t729 * t6154 * t2579;
    let t24683 = t724 * t1456 * t2413;
    let t24687 = t2594 * t1456 * t2405;
    let t24690 = t1882 * t6101;
    (t24669, t24670, t24673, t24675, t24679, t24683, t24687, t24690)
}
