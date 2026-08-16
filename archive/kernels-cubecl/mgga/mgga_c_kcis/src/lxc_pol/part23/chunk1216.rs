//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1216/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1216<F: Float>(t5913: F, t94748: F, t2043: F, t4244: F, t27526: F, t28594: F, t11776: F, t2062: F, t8191: F, t94743: F, t17417: F, t28624: F) -> (F, F, F, F, F, F) {
    let t97807 = t94748 * t5913;
    let t97809 = t4244 * t2043;
    let t97811 = t28594 * t27526;
    let t97813 = t11776 * t2062;
    let t97815 = t94743 * t8191;
    let t97817 = t28624 * t17417;
    (t97807, t97809, t97811, t97813, t97815, t97817)
}
