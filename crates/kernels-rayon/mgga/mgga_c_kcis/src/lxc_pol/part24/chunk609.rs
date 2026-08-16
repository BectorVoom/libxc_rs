//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 609/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk609(t2888: f64, t6517: f64, t1662: f64, t1704: f64, t2894: f64, t2899: f64, t6272: f64, t993: f64, t6276: f64, t994: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6518 = t2888 * t6517;
    let t6521 = t1662 * t1704;
    let t6522 = t2894 * t6521;
    let t6525 = t2899 * t6272;
    let t6526 = t993 * t6525;
    let t6529 = t994 * t6276;
    let t6530 = t993 * t6529;
    let t6533 = t1704 * t1704;
    (t6518, t6521, t6522, t6525, t6526, t6529, t6530, t6533)
}
