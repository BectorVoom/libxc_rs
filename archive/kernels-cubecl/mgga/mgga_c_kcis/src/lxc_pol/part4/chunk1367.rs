//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1367/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1367<F: Float>(t1517: F, t2645: F, t5987: F, t1979: F, t3754: F, t2642: F, t4219: F, t2018: F, t456: F, t3820: F, t562: F, t143: F, t16349: F) -> (F, F, F, F, F) {
    let t17605 = t1517 * t5987 * t2645;
    let t17608 = t1979 * t3754;
    let t17610 = t4219 * t17608 * t2642;
    let t17613 = t2018 * t456;
    let t17627 = t562 * t3820;
    let t17630 = t16349 * t143;
    (t17605, t17610, t17613, t17627, t17630)
}
