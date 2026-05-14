//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1332/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1332<F: Float>(t10016: F, t2139: F, t2294: F, t10034: F, t6118: F, t1569: F, t2526: F, t10049: F, t20997: F, t20998: F, t2122: F, t2124: F, t2133: F, t24695: F, t2531: F, t2551: F, t25606: F, t2625: F, t2654: F, t27831: F, t27834: F, t27858: F, t27885: F, t27914: F, t32319: F, t32516: F, t360: F, t6293: F, t8289: F, t8773: F, t8842: F, t9317: F, t9995: F) -> (F, F) {
    let t32565 = t2139 * t2294 * t10016;
    let t32568 = t6118 * t10034;
    let t32590 = t1569 * t2526;
    let t32596 = -0.13002332610081402845e0 * t8289 * t9995 + 0.54878743191129263322e-1 * t2122 * t2124 * t32516 * t2551 - 0.13869154784086829701e1 * t27831 + 0.69345773920434148506e0 * t27834 - 0.10401866088065122276e1 * t32565 - 0.34672886960217074253e0 * t27858 - 0.38415120233790484324e0 * t32568 - 0.49390868872016336989e0 * t6293 * t2124 * t8842 * t2531 + 0.23404198698146525121e1 * t25606 * t360 * t8773 * t2654 + 0.2600466522016280569e0 * t20997 * t360 * t32319 * t20998 + 0.31205598264195366828e1 * t24695 * t360 * t8773 * t2625 + 0.13002332610081402845e0 * t2133 * t360 * t27914 * t10049 - 0.32927245914677557992e0 * t2122 * t2124 * t9317 * t32590 + 0.19207560116895242163e0 * t27885;
    (t32590, t32596)
}
