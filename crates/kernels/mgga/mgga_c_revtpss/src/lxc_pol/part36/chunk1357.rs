//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1357/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1357<F: Float>(t104721: F, t104927: F, t104988: F, t104990: F, t104999: F, t112373: F, t112480: F, t1785: F, t1791: F, t2137: F, t2138: F, t24244: F, t24679: F, t24699: F, t24787: F, t26867: F, t29047: F, t29048: F, t29086: F, t30815: F, t467: F, t484: F, t6601: F, t6611: F, t6640: F, t6647: F, t8184: F) -> F {
    let t116290 = F::new(0.15244095330869239812e-2) * t104988 + t104990 / F::new(432.0) - F::new(0.10620053080505570402e0) * t467 * t2137 * t24679 * t484 + F::new(0.42874018118069736972e-3) * t24699 * t2138 * t484 + F::new(0.43445671692977333464e-1) * t1785 * t30815 * t484 - F::new(0.68598428988911579154e-2) * t6601 * t8184 * t484 - F::new(0.28582678745379824648e-3) * t104999 + F::new(0.91464571985215438873e-2) * t104721 * t6640 - F::new(0.85748036236139473944e-3) * t26867 * t24787 - F::new(0.12862205435420921092e-2) * t29086 * t6647 + F::new(0.25724410870841842183e-2) * t104927 * t6611 - F::new(0.12862205435420921092e-2) * t112373 * t1791 + F::new(0.13719685797782315831e-1) * t112480 * t1791 - t29047 * t29048 * t24244 / F::new(48.0);
    t116290
}
