//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 830/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk830<F: Float>(t3605: F, t721: F, t1916: F, t1938: F, t1955: F, t1977: F, t2834: F, t2853: F, t5830: F, t5838: F, t5871: F, t7315: F, t7494: F, t9410: F, t9413: F, t9416: F, t9419: F, t9423: F, t9426: F, t9430: F, t9437: F, t9440: F, t9443: F) -> (F, F) {
    let t9446 = t3605 * t721;
    let t9449 = 6.0 * t1938 * t9410 - 4.0 * t1916 * t9413 - 0.19298375398431042081e3 * t5830 * t9416 - 2.0 * t1916 * t9419 + 0.32163958997385070134e2 * t1938 * t9423 + 0.64327917994770140268e2 * t1938 * t9426 + 0.2069040516770936012e4 * t5871 * t9430 - 0.23392894490538584828e1 * t7494 * t2834 + 0.34631718211362927517e2 * t7315 * t2853 + 0.35089341735807877242e1 * t1977 * t9437 - 0.23392894490538584828e1 * t1955 * t9440 - 0.10389515463408878255e3 * t5838 * t9443 - 0.11696447245269292414e1 * t1955 * t9446;
    (t9446, t9449)
}
