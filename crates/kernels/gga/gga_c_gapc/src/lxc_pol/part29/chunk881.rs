//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 881/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk881<F: Float>(t11905: F, t7595: F, t8135: F, t11356: F, t3402: F, t9934: F, t1084: F, t9865: F, t291: F, t8448: F, t1971: F, t9846: F, t11863: F, t11865: F, t11867: F, t11870: F, t11873: F, t11879: F, t11882: F, t11885: F, t11890: F, t11893: F, t11895: F, t11898: F, t11900: F, t11903: F) -> (F, F, F, F, F) {
    let t11906 = t11905 * t7595;
    let t11908 = t11905 * t8135;
    let t11910 = t3402 * t11356;
    let t11911 = t11910 * t9934;
    let t11913 = t1084 * t11356;
    let t11914 = t11913 * t9865;
    let t11916 = t8448 * t291;
    let t11917 = t1971 * t11916;
    let t11918 = t1084 * t11917;
    let t11919 = t11918 * t9846;
    let t11921 = -0.10120442708333333334e-4 * t11863 - 0.10120442708333333334e-4 * t11865 + 0.54106179813099907243e-4 * t11867 + 0.76936424826321944924e-9 * t11870 + 0.16882049790461501058e-6 * t11873 + 0.50551591594011046914e-6 * t11879 - 0.45289771048911752714e-7 * t11882 - 0.26419033111865189083e-7 * t11885 + 0.15837668668010950386e-5 * t11890 - 0.16882049790461501058e-6 * t11893 - 0.16882049790461501058e-6 * t11895 - 0.10005428175813516294e-7 * t11898 - 0.10023717063086516253e-6 * t11900 + 0.45289771048911752714e-7 * t11903 - 0.90579542097823505428e-7 * t11906 + 0.90579542097823505428e-7 * t11908 - 0.52756405595192190805e-8 * t11911 - 0.10551281119038438161e-7 * t11914 + 0.19645612283222543108e-8 * t11919;
    (t11910, t11913, t11917, t11918, t11921)
}
