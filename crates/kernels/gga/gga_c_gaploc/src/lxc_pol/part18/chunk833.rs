//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 833/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk833<F: Float>(t1901: F, t8549: F, t8512: F, t2580: F, t8470: F, t8474: F, t7226: F, t8612: F, t123: F, t8519: F, t734: F, t1022: F, t2101: F, t1891: F, t8729: F, t1025: F, t1841: F, t1897: F, t1908: F, t2508: F, t2933: F, t2960: F, t5227: F, t5269: F, t5524: F, t7129: F, t7137: F, t7299: F, t7303: F, t7309: F, t7315: F, t7318: F) -> (F, F, F) {
    let t8991 = t1901 * t8549;
    let t8994 = t1901 * t8512;
    let t8997 = t2580 * t8470;
    let t9000 = t2580 * t8474;
    let t9003 = t7226 * t8612;
    let t9006 = t8519 * t123;
    let t9007 = t9006 * t734;
    let t9014 = t2101 * t1022;
    let t9015 = t9014 * t1891;
    let t9020 = t1901 * t8729;
    let t9030 = 0.34180116578409885707e-2 * t1908 * t1025 + 0.15381052460284448567e-1 * t1897 * t8991 + 0.76905262301422242837e-2 * t1897 * t8994 + 0.30762104920568897134e-1 * t2508 * t8997 + 0.15381052460284448567e-1 * t2508 * t9000 - 0.46143157380853345701e-1 * t2508 * t9003 - 0.17090058289204942853e-2 * t1841 * t9007 + 0.8545029144602471425e-3 * t5524 * t2933 - 0.17090058289204942853e-2 * t5227 * t2933 + 0.92286314761706691403e-1 * t2508 * t9015 + 0.30762104920568897134e-1 * t7129 * t2960 - 0.15381052460284448567e-1 * t5269 * t9020 + 0.41016139894091862847e-1 * t7137 * t2960 + 0.1281754371690370714e-2 * t7299 - 0.1922631557535556071e-2 * t7303 - 0.1281754371690370714e-2 * t7309 + 0.1281754371690370714e-2 * t7315 + 0.64087718584518535698e-3 * t7318;
    (t9006, t9014, t9030)
}
