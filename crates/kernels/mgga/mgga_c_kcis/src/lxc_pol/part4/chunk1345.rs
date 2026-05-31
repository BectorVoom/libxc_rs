//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1345/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1345<F: Float>(t1555: F, t6048: F, t4189: F, t2069: F, t4310: F, t17253: F, t552: F, t577: F, t585: F, t1489: F, t5880: F, t4293: F) -> (F, F, F, F, F) {
    let t17323 = t6048 * t1555;
    let t17325 = F::cast_from(4.0_f64) * t4189 * t17323;
    let t17326 = t2069 * t4310;
    let t17328 = F::cast_from(2.0_f64) * t4189 * t17326;
    let t17329 = t17253 * t552;
    let t17330 = t17329 * t577;
    let t17331 = t17330 * t585;
    let t17333 = t5880 * t1489;
    let t17334 = t4293 * t17333;
    (t17325, t17328, t17331, t17333, t17334)
}
