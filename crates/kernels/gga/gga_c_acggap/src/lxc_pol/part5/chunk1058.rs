//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1058/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1058<F: Float>(t1165: F, t16020: F, t3194: F, t530: F, t3409: F, t5209: F, t1456: F, t3228: F, t1462: F, t1451: F, t3237: F, t4728: F, t997: F) -> (F, F, F, F, F, F) {
    let t18620 = t3194 * t1165 * t530 * t16020;
    let t18622 = t3409 * t5209;
    let t18628 = t3228 * t1456;
    let t18633 = t3228 * t1462;
    let t18647 = t3237 * t1451;
    let t18649 = t997 * t4728;
    (t18620, t18622, t18628, t18633, t18647, t18649)
}
