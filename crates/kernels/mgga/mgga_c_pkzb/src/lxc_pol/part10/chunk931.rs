//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 931/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk931<F: Float>(t1956: F, t2852: F, t2751: F, t5734: F, t1916: F, t1938: F, t1955: F, t2834: F, t2853: F, t5830: F, t5835: F, t5838: F, t5871: F, t5903: F, t7231: F, t7234: F, t7237: F, t7241: F, t7244: F, t7248: F, t7255: F, t7258: F) -> (F, F, F) {
    let t7261 = t2852 * t1956;
    let t7265 = 4.0 * t5734 * t2751;
    let t7266 = -4.0 * t1916 * t7231 - 2.0 * t1916 * t7234 - 0.19298375398431042081e3 * t5830 * t7237 + 0.64327917994770140268e2 * t1938 * t7241 + 0.32163958997385070134e2 * t1938 * t7244 + 0.2069040516770936012e4 * t5871 * t7248 - 0.23392894490538584828e1 * t5903 * t2834 + 0.34631718211362927518e2 * t5835 * t2853 - 0.23392894490538584828e1 * t1955 * t7255 - 0.11696447245269292414e1 * t1955 * t7258 - 0.10389515463408878255e3 * t5838 * t7261 + t7265;
    (t7261, t7265, t7266)
}
