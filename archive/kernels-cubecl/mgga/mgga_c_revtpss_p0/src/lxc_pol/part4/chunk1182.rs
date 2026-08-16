//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1182/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1182<F: Float>(t14609: F, t14610: F, t14612: F, t14630: F, t225: F, t73: F, t830: F, t1544: F, t2475: F, t2394: F, t4343: F, t853: F) -> (F, F, F, F, F) {
    let t14633 = (t14609 + t14610 + t14612 + t14630) * t225;
    let t14643 = t830 * t73;
    let t14648 = t2475 * t1544;
    let t14649 = t14648 * t2394;
    let t14652 = t853 * t4343;
    (t14633, t14643, t14648, t14649, t14652)
}
