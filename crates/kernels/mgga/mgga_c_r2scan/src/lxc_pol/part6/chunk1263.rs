//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1263/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1263<F: Float>(t19000: F, t19002: F, t19004: F, t19014: F, t2333: F, t8299: F, t1048: F, t795: F, t1411: F, t2747: F, t1385: F, t406: F, t7124: F, t4990: F, t963: F, t19013: F, t19032: F, t19037: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t23750 = 0.31168546390226634765e3 * t19000;
    let t23751 = 0.35089341735807877242e1 * t19002;
    let t23752 = 36.0 * t19004;
    let t23753 = 8.0 * t19014;
    let t23754 = t8299 * t2333;
    let t23757 = 3.0 * t1048 * t23754 * t795;
    let t23758 = t2747 * t1411;
    let t23759 = 0.17544670867903938621e1 * t23758;
    let t23760 = t2747 * t1385;
    let t23761 = 0.51947577317044391276e2 * t23760;
    let t23763 = 24.0 * t406 * t7124;
    let t23764 = t963 * t4990;
    let t23765 = 0.35089341735807877242e1 * t23764;
    let t23766 = -t23750 - t23751 + t23752 - t19013 - t23753 + t23757 + t23759 + t23761 - t23763 - t19032 + t23765 - t19037;
    (t23750, t23751, t23752, t23753, t23757, t23759, t23761, t23763, t23765, t23766)
}
