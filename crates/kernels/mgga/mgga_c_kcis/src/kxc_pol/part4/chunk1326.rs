//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1326/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1326<F: Float>(t1419: F, t17020: F, t12048: F, t5796: F, t1401: F, t5808: F, t1409: F, t16533: F, t1951: F, t2642: F, t1650: F, t4035: F) -> (F, F, F, F, F, F) {
    let t17021 = t17020 * t1419;
    let t17024 = t12048 * t5796;
    let t17027 = F::cast_from(0.93706135855523581992e-2_f64) * t1401 * t5808;
    let t17028 = t1409 * t16533;
    let t17037 = t1951 * t2642;
    let t17040 = t4035 * t1650;
    (t17021, t17024, t17027, t17028, t17037, t17040)
}
