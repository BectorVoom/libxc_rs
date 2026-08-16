//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 957/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk957<F: Float>(t4962: F, t9938: F, t991: F, t1071: F, t1704: F, t2630: F, t2894: F, t1000: F, t4951: F, t1003: F, t4621: F, t13475: F, t4947: F) -> (F, F, F, F, F) {
    let t14536 = t9938 * t4962;
    let t14538 = t991 * t14536 / F::cast_from(432.0_f64);
    let t14542 = t1704 * t1071 * t2630;
    let t14543 = t2894 * t14542;
    let t14546 = t4951 * t1000;
    let t14547 = t4621 * t1003;
    let t14548 = t14546 * t14547;
    let t14551 = t4947 * t13475;
    (t14538, t14543, t14546, t14548, t14551)
}
