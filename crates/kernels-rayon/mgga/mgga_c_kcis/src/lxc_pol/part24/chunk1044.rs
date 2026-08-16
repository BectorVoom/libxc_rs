//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1044/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1044(t1876: f64, t914: f64, t2169: f64, t7673: f64, t8024: f64, t8122: f64, t911: f64, t1655: f64, t7671: f64, t1658: f64, t7827: f64, t233: f64) -> (f64, f64, f64, f64, f64) {
    let t27734 = t914 * t1876;
    let t27735 = t2169 * t27734;
    let t27737 = t7673 * t8024;
    let t27739 = t911 * t8122;
    let t27741 = t1655 * t7671;
    let t27743 = t1658 * t7827;
    let t27744 = t233 * t27743;
    (t27735, t27737, t27739, t27741, t27744)
}
