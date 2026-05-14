//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1302/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1302<F: Float>(t1458: F, t34831: F, t1520: F, t34843: F, t41218: F, t33618: F, t6394: F, t14287: F, t34846: F, t14294: F, t8189: F, t9509: F, t2732: F, t27431: F, t4170: F, t8286: F) -> (F, F, F, F, F, F, F) {
    let t118663 = t34831 * t1458;
    let t118664 = t118663 * t1520;
    let t118667 = 24.0 * t41218 * t34843 * t1520;
    let t118669 = 2.0 * t33618 * t6394;
    let t118671 = 4.0 * t14287 * t34846;
    let t118674 = 6.0 * t14294 * t9509 * t8189;
    let t118677 = 2.0 * t4170 * t2732 * t27431;
    let t118680 = 2.0 * t4170 * t9509 * t8286;
    (t118664, t118667, t118669, t118671, t118674, t118677, t118680)
}
