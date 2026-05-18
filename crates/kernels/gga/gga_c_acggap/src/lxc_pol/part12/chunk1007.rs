//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1007/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1007<F: Float>(t5213: F, t7822: F, t157: F, t33750: F, t1165: F, t2068: F, t604: F, t30268: F, t8775: F, t30105: F, t8952: F, t7839: F, t8739: F) -> (F, F, F, F, F, F) {
    let t33974 = t7822 * t5213;
    let t33976 = t33750 * t157;
    let t33979 = t2068 * t1165 * t604 * t33976;
    let t33982 = t30268 * t8775;
    let t33984 = t30105 * t8952;
    let t33986 = t7839 * t8739;
    (t33974, t33976, t33979, t33982, t33984, t33986)
}
