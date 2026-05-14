//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 936/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk936<F: Float>(t14516: F, t991: F, t2872: F, t4936: F, t1699: F, t9916: F, t4962: F, t9938: F, t1000: F, t4951: F, t291: F, t1014: F, t4925: F, t4768: F, t978: F, t2861: F, t4986: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14518 = t991 * t14516 / 72.0;
    let t14527 = t2872 * t4936 / 162.0;
    let t14528 = t9916 * t1699;
    let t14529 = t991 * t14528;
    let t14536 = t9938 * t4962;
    let t14538 = t991 * t14536 / 432.0;
    let t14546 = t4951 * t1000;
    let t14554 = t4951 * t291;
    let t14567 = t1014 * t4925;
    let t14568 = 0.33163888888888888888e-2 * t14567;
    let t14570 = t4768 * t978;
    let t14576 = t2861 * t4986;
    (t14518, t14527, t14529, t14538, t14546, t14554, t14567, t14568, t14570, t14576)
}
