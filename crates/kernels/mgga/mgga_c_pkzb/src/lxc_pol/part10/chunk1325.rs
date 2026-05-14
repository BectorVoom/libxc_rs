//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1325/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1325<F: Float>(t3577: F, t5873: F, t20683: F, t7279: F, t663: F, t9343: F, t685: F, t17475: F, t17478: F, t17633: F, t1916: F, t1917: F, t1932: F, t1938: F, t1956: F, t1971: F, t20905: F, t21087: F, t21179: F, t2801: F, t2819: F, t2820: F, t3578: F, t3591: F, t5825: F, t5830: F, t5835: F, t5845: F, t5871: F, t5897: F, t5903: F, t7231: F, t7241: F, t7324: F, t7407: F, t7486: F, t9401: F, t9402: F, t9410: F, t9413: F, t9422: F, t9437: F, t9440: F) -> (F, F, F) {
    let t26274 = t3577 * t5873;
    let t26282 = 0.38596750796862084161e3 * t20683 * t7279;
    let t26283 = t9343 * t663;
    let t26285 = 2.0 * t26283 * t685;
    let t26313 = -2.0 * t1916 * t3578 * t1932 - 0.19298375398431042081e3 * t5830 * t9422 * t1917 + 0.32163958997385070134e2 * t1938 * t9422 * t1932 + 0.2069040516770936012e4 * t5871 * t26274 * t1917 + 0.64327917994770140268e2 * t1938 * t2819 * t7407 + t26282 - t26285 + 24.0 * t20905 * t7231 - 8.0 * t21087 * t2801 + 0.12865583598954028054e3 * t21179 * t2820 - 8.0 * t7486 * t7231 + 0.12865583598954028054e3 * t7324 * t7241 + 12.0 * t5825 * t9410 - 8.0 * t5897 * t9413 + 0.20508037716432813316e4 * t17633 * t9402 + 0.10254018858216406658e4 * t5845 * t9401 * t1971 + 0.91082604192152556044e5 * t17475 * t3591 * t17478 * t1956 + 0.70178683471615754484e1 * t5835 * t9437 - 0.46785788981077169656e1 * t5903 * t9440;
    (t26282, t26285, t26313)
}
