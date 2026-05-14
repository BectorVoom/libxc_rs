//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1284/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1284<F: Float>(t2023: F, t46: F, t616: F, t2004: F, t2900: F, t2916: F, t655: F, t300: F, t3638: F, t779: F, t2104: F, t54: F, t9257: F, t9259: F, t154: F, t17874: F, t17881: F, t17897: F, t17902: F, t17905: F, t2031: F, t2036: F, t21542: F, t21567: F, t25290: F, t25315: F, t276: F, t2774: F, t2899: F, t2923: F, t295: F, t5729: F, t5956: F, t5984: F, t742: F, t7700: F, t7736: F, t7742: F, t7768: F, t9260: F) -> (F, F, F, F, F) {
    let t25323 = t2023 * t46;
    let t25324 = t25323 * t616;
    let t25326 = t2004 * t2900;
    let t25331 = t655 * t2916;
    let t25337 = t300 * t779 * t3638;
    let t25351 = t2104 * t54 * t9257 * t9259;
    let t25353 = -2.0 / 81.0 * t21542 - 5.0 / 243.0 * t17874 - t17881 - t25290 / 144.0 - t276 * t154 * t742 * t25315 / 96.0 - 0.67751534803863288054e-3 * t17897 - t17902 - 0.16090989515917530913e-2 * t17905 - 0.51448821741683684366e-2 * t2036 * t295 * t25324 * t25326 * t2923 * t2774 - 0.34299214494455789578e-2 * t2899 * t7700 * t2031 * t25331 - 0.51448821741683684367e-2 * t7736 * t25337 * t5956 * t7768 + 0.51448821741683684367e-2 * t7742 * t25337 * t5729 * t7768 + 0.34299214494455789578e-2 * t21567 - 0.27439371595564631662e-1 * t5984 * t9260 + 0.34299214494455789578e-2 * t25351;
    (t25324, t25326, t25331, t25337, t25353)
}
