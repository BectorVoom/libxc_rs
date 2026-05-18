//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 911/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk911<F: Float>(t2268: F, t2854: F, t29853: F, t6320: F, t39764: F, t39766: F, t12770: F, t484: F, t42726: F, t42730: F, t42733: F, t42737: F, t42739: F, t42742: F, t42743: F, t42745: F, t42748: F, t42751: F, t42756: F, t42759: F, t42763: F, t42767: F) -> F {
    let t42771 = F::new(0.17073003981405689759e0) * t2268 * t6320 * t2854 * t29853;
    let t42772 = F::new(0.31616674039640166221e-2) * t39764;
    let t42773 = F::new(0.31616674039640166221e-2) * t39766;
    let t42774 = t484 * t12770;
    let t42776 = F::new(0.31616674039640166221e-2) * t42726 - t42730 + t42733 - t42737 + t42739 + t42742 + F::new(0.15176003539027279787e0) * t42743 + F::new(0.23712505529730124666e-2) * t42745 + F::new(0.23712505529730124666e-2) * t42748 + F::new(0.1707300398140568976e0) * t42751 - t42756 + F::new(0.56910013271352299198e-1) * t42759 + t42763 + t42767 - t42771 - t42772 + t42773 - F::new(0.31616674039640166221e-2) * t42774;
    t42776
}
