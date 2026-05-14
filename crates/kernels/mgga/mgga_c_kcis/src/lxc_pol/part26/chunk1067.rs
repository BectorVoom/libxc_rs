//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1067/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1067<F: Float>(t29659: F, t448: F, t7570: F, t2233: F, t447: F, t6887: F, t2234: F, t2272: F, t6896: F, t446: F, t1881: F, t8141: F, t8130: F, t8133: F, t8256: F, t637: F, t6895: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29660 = t29659 / 16.0;
    let t29662 = t448 * t7570;
    let t29663 = t2233 * t29662;
    let t29664 = t29663 / 16.0;
    let t29665 = t6887 * t447;
    let t29666 = t29665 * t2234;
    let t29667 = t29666 / 8.0;
    let t29668 = t6896 * t2272;
    let t29669 = t446 * t29668;
    let t29670 = t29669 / 16.0;
    let t29671 = t1881 * t8141;
    let t29672 = t29671 / 8.0;
    let t29673 = t8130 * t8133;
    let t29674 = t29673 / 8.0;
    let t29675 = t1881 * t8256;
    let t29676 = t29675 / 8.0;
    let t29677 = t6895 * t637;
    (t29660, t29664, t29665, t29667, t29670, t29672, t29674, t29676, t29677)
}
