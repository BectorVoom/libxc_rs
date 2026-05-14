//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 829/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk829<F: Float>(t734: F, t8871: F, t7289: F, t8769: F, t161: F, t8773: F, t1845: F, t5396: F, t8756: F, t1716: F, t2936: F, t1035: F, t1836: F, t1024: F, t2060: F, t1025: F, t1030: F, t1841: F, t1850: F, t1897: F, t1935: F, t1939: F, t2508: F, t2928: F, t2951: F, t2964: F, t5288: F, t5293: F, t650: F, t681: F, t7066: F, t8868: F) -> (F, F) {
    let t8872 = t8871 * t734;
    let t8875 = t7289 * t8769;
    let t8878 = t8773 * t161;
    let t8879 = t8878 * t1845;
    let t8882 = t5396 * t8756;
    let t8902 = t2936 * t1716;
    let t8905 = t1035 * t1836;
    let t8908 = t2060 * t1024;
    let t8911 = 0.17090058289204942853e-2 * t1850 * t8868 - 0.17090058289204942853e-2 * t1841 * t8872 - 0.34180116578409885705e-2 * t1841 * t8875 + 0.51270174867614828558e-2 * t1841 * t8879 - 0.17090058289204942853e-2 * t1850 * t8882 - 0.1281754371690370714e-2 * t7066 + 0.20508069947045931424e-1 * t1939 * t1025 + 0.20508069947045931424e-1 * t650 * t2928 + 0.76905262301422242837e-2 * t1935 * t1025 + 0.15381052460284448567e-1 * t681 * t2928 - 0.20508069947045931424e-1 * t1939 * t1030 - 0.20508069947045931424e-1 * t650 * t2964 + 0.15381052460284448567e-1 * t5288 * t2951 + 0.20508069947045931424e-1 * t5293 * t2951 - 0.23071578690426672851e-1 * t2508 * t8902 - 0.76905262301422242837e-2 * t1897 * t8905 + 0.76905262301422242837e-2 * t2508 * t8908;
    (t8878, t8911)
}
