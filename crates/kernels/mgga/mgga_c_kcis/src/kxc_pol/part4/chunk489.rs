//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 489/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk489<F: Float>(t2046: F, t572: F, t571: F, t1981: F, t552: F, t577: F, t585: F, t2001: F, t584: F, t583: F, t1546: F, t487: F, t579: F) -> (F, F, F, F, F, F, F, F) {
    let t2047 = t572 * t2046;
    let t2048 = t571 * t2047;
    let t2050 = t1981 * t552;
    let t2051 = t2050 * t577;
    let t2052 = t2051 * t585;
    let t2054 = t584 * t2001;
    let t2055 = t583 * t2054;
    let t2056 = t1546 * t2055;
    let t2058 = t579 * t487;
    (t2047, t2048, t2050, t2051, t2052, t2055, t2056, t2058)
}
