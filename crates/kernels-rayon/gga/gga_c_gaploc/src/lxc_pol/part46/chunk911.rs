//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 911/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk911(t2268: f64, t2854: f64, t29853: f64, t6320: f64, t39764: f64, t39766: f64, t12770: f64, t484: f64, t42726: f64, t42730: f64, t42733: f64, t42737: f64, t42739: f64, t42742: f64, t42743: f64, t42745: f64, t42748: f64, t42751: f64, t42756: f64, t42759: f64, t42763: f64, t42767: f64) -> f64 {
    let t42771 = 0.17073003981405689759e0_f64 * t2268 * t6320 * t2854 * t29853;
    let t42772 = 0.31616674039640166221e-2_f64 * t39764;
    let t42773 = 0.31616674039640166221e-2_f64 * t39766;
    let t42774 = t484 * t12770;
    let t42776 = 0.31616674039640166221e-2_f64 * t42726 - t42730 + t42733 - t42737 + t42739 + t42742 + 0.15176003539027279787e0_f64 * t42743 + 0.23712505529730124666e-2_f64 * t42745 + 0.23712505529730124666e-2_f64 * t42748 + 0.1707300398140568976e0_f64 * t42751 - t42756 + 0.56910013271352299198e-1_f64 * t42759 + t42763 + t42767 - t42771 - t42772 + t42773 - 0.31616674039640166221e-2_f64 * t42774;
    t42776
}
