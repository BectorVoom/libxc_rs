//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 485/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk485<F: Float>(t1842: F, t234: F, t732: F, t741: F, t750: F, t625: F, t626: F, t645: F, t124: F, t182: F, t190: F, t406: F, t726: F, t58: F, t583: F) -> (F, F, F, F, F, F, F, F) {
    let t1844 = 0.34631718211362927518e2 * t234 * t1842;
    let t1845 = t732 * t741;
    let t1847 = t732 * t750;
    let t1851 = 0.35616666666666666666e-1 * t625 * t626 * t645;
    let t1853 = t124 * t182;
    let t1856 = 0.23744444444444444444e-1 * t625 * t1853 * t190;
    let t1858 = 8.0 * t406 * t726;
    let t1859 = t583 * t58;
    (t1844, t1845, t1847, t1851, t1853, t1856, t1858, t1859)
}
