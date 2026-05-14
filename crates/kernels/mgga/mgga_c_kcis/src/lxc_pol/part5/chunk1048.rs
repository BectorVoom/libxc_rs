//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1048/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1048<F: Float>(t1662: F, t4984: F, t9517: F, t3200: F, t1767: F, t3217: F, t4813: F, t3202: F, t14627: F, t2855: F, t6326: F, t1021: F, t2842: F, t1022: F, t18681: F, t1020: F) -> (F, F, F, F) {
    let t19571 = t1662 * t4984;
    let t19572 = t9517 * t19571;
    let t19573 = t3200 * t19572;
    let t19575 = t3217 * t1767;
    let t19576 = t19575 * t4813;
    let t19577 = t3202 * t19576;
    let t19578 = t14627 * t19577;
    let t19580 = t2855 * t6326;
    let t19581 = t1021 * t19580;
    let t19582 = t2842 * t19581;
    let t19584 = t1022 * t18681;
    let t19585 = t1021 * t19584;
    let t19586 = t1020 * t19585;
    (t19573, t19578, t19582, t19586)
}
