//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 803/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk803<F: Float>(t14496: F, t291: F, t1245: F, t4967: F, t991: F, t2872: F, t4936: F, t1699: F, t9916: F, t4962: F, t9938: F, t1000: F, t4951: F) -> (F, F, F, F, F, F) {
    let t14497 = t14496 * t291;
    let t14516 = t1245 * t4967;
    let t14518 = t991 * t14516 / F::cast_from(72.0_f64);
    let t14527 = t2872 * t4936 / F::cast_from(162.0_f64);
    let t14528 = t9916 * t1699;
    let t14529 = t991 * t14528;
    let t14536 = t9938 * t4962;
    let t14538 = t991 * t14536 / F::cast_from(432.0_f64);
    let t14546 = t4951 * t1000;
    (t14497, t14518, t14527, t14529, t14538, t14546)
}
