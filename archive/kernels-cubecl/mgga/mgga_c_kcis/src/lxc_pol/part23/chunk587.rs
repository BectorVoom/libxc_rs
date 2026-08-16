//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 587/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk587<F: Float>(t5654: F, t5662: F, t4170: F, t5661: F, t1307: F, t2038: F, t4162: F, t4160: F, t1489: F, t2011: F, t1495: F, t1468: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5663 = t5662 * t5654;
    let t5664 = t4170 * t5663;
    let t5665 = t5661 * t5664;
    let t5667 = t2038 * t1307;
    let t5668 = t4162 * t5667;
    let t5669 = t4160 * t5668;
    let t5671 = t2011 * t1489;
    let t5672 = t1495 * t5671;
    let t5673 = t1468 * t5672;
    (t5663, t5664, t5665, t5667, t5668, t5669, t5671, t5672, t5673)
}
