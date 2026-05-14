//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1154/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1154<F: Float>(t100680: F, t100683: F, t101084: F, t101464: F, t101469: F, t26685: F, t27808: F, t27832: F, t27895: F, t27911: F, t27915: F, t95524: F, t96218: F, t96231: F, t15573: F, t28947: F) -> (F, F) {
    let t101476 = -0.18550940104166666667e-3 * t95524 * t27911 - 0.13901041666666666667e-2 * t27832 * t27911 - 0.27802083333333333334e-2 * t27832 * t27808 - 0.15445601851851851852e-3 * t101464 + t96218 + 0.18550940104166666667e-3 * t27895 * t27915 - t96231 - 0.92754700520833333333e-4 * t26685 * t101469 - 0.2782641015625e-3 * t26685 * t101084 + 0.22109259259259259259e-2 * t100680 - 0.49745833333333333332e-2 * t100683;
    let t101501 = t15573 * t28947;
    (t101476, t101501)
}
