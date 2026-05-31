//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1065/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1065<F: Float>(t1927: F, t26204: F, t1923: F, t2048: F, t25102: F, t25110: F, t25114: F, t25117: F, t25120: F, t25150: F, t25159: F, t25162: F, t26170: F, t26172: F, t26175: F, t26180: F, t26182: F, t26185: F, t26187: F, t26190: F, t6954: F, t6960: F, t6963: F, t7343: F, t7352: F) -> (F, F) {
    let t26205 = t26204 * t1927;
    let t26207 = F::cast_from(88.0_f64) / F::cast_from(27.0_f64) * t1923 * t26205;
    let t26208 = t25150 * t2048 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6954 * t7352 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t26170 + t1923 * t26172 / F::cast_from(3.0_f64) + F::cast_from(10.0_f64) * t26175 * t25159 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t26180 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t25162 * t26182 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t26185 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t26187 * t6960 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t26190 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t25102 * t2048 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t7343 * t25110 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7343 * t25114 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t25117 * t2048 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t25120 * t2048 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t6963 * t7352 + t26207;
    (t26205, t26208)
}
