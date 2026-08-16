//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1163/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1163<F: Float>(t14627: F, t19577: F, t2855: F, t6326: F, t1021: F, t2842: F, t1022: F, t18681: F, t1020: F, t1133: F, t6496: F, t9546: F) -> (F, F, F, F, F) {
    let t19578 = t14627 * t19577;
    let t19580 = t2855 * t6326;
    let t19581 = t1021 * t19580;
    let t19582 = t2842 * t19581;
    let t19584 = t1022 * t18681;
    let t19585 = t1021 * t19584;
    let t19586 = t1020 * t19585;
    let t19588 = t6496 * t1133;
    let t19589 = t9546 * t19588;
    (t19578, t19582, t19586, t19588, t19589)
}
