//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 797/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk797<F: Float>(t167: F, t994: F, t4951: F, t1705: F, t25: F, t285: F, t1704: F, t330: F, t829: F, t2894: F, t2909: F, t1003: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4952 = t994 * t167;
    let t4953 = t4951 * t4952;
    let t4958 = t25 * t1705;
    let t4959 = t285 * t4958;
    let t4961 = t1704 * t330;
    let t4962 = t4961 * t829;
    let t4963 = t2894 * t4962;
    let t4966 = t2909 * t1704;
    let t4967 = t4966 * t1003;
    (t4952, t4953, t4958, t4959, t4961, t4962, t4963, t4966, t4967)
}
