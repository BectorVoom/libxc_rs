//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1326/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1326<F: Float>(t1979: F, t9203: F, t1940: F, t9493: F, t1899: F, t1901: F, t25968: F, t1915: F, t3559: F, t17391: F, t17514: F, t17624: F, t1916: F, t1918: F, t1938: F, t1955: F, t1977: F, t21143: F, t21156: F, t2834: F, t2853: F, t5825: F, t5835: F, t5897: F, t5903: F, t702: F, t721: F, t7255: F, t7300: F, t7315: F, t7494: F, t9416: F, t9419: F, t9423: F, t9426: F, t9430: F, t9443: F, t9446: F, t9452: F, t9455: F, t9465: F, t9494: F) -> (F, F) {
    let t26323 = t9203 * t1979;
    let t26336 = t9493 * t1940;
    let t26356 = 0.32163958997385070134e2 * t1899 * t25968 * t1901;
    let t26357 = t3559 * t1915;
    let t26360 = -0.2077903092681775651e3 * t17624 * t9443 - 0.23392894490538584828e1 * t5903 * t9446 + 0.34631718211362927518e2 * t5835 * t9452 - 0.23392894490538584828e1 * t1955 * t9465 * t721 + 0.34631718211362927518e2 * t1977 * t26323 * t721 + 0.69263436422725855036e2 * t5835 * t9455 - 4.0 * t5897 * t9419 + 0.64327917994770140268e2 * t5825 * t9423 - 4.0 * t1916 * t9494 * t702 + 0.64327917994770140268e2 * t1938 * t26336 * t702 - 0.38596750796862084162e3 * t17391 * t9416 + 0.12865583598954028054e3 * t5825 * t9426 + 0.4138081033541872024e4 * t17514 * t9430 - 0.46785788981077169656e1 * t21143 * t2834 + 0.69263436422725855034e2 * t21156 * t2853 - 0.46785788981077169656e1 * t7494 * t7255 + 0.69263436422725855034e2 * t7315 * t7300 - t26356 - 2.0 * t26357 * t1918;
    (t26356, t26360)
}
