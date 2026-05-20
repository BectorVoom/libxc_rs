//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1703/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1703<F: Float>(t1923: F, t26205: F, t2048: F, t25102: F, t25110: F, t25114: F, t25117: F, t25120: F, t25150: F, t25159: F, t25162: F, t26170: F, t26172: F, t26175: F, t26180: F, t26182: F, t26185: F, t26187: F, t26190: F, t6954: F, t6960: F, t6963: F, t7343: F, t7352: F) -> (F, F) {
    let t26207 = F::new(88.0) / F::new(27.0) * t1923 * t26205;
    let t26208 = t25150 * t2048 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t6954 * t7352 - F::new(16.0) / F::new(9.0) * t26170 + t1923 * t26172 / F::new(3.0) + F::new(10.0) * t26175 * t25159 + F::new(80.0) / F::new(9.0) * t26180 + F::new(20.0) / F::new(3.0) * t25162 * t26182 + F::new(32.0) / F::new(9.0) * t26185 - F::new(10.0) / F::new(3.0) * t26187 * t6960 - F::new(16.0) / F::new(9.0) * t26190 - F::new(4.0) / F::new(3.0) * t25102 * t2048 - F::new(10.0) / F::new(3.0) * t7343 * t25110 - F::new(5.0) / F::new(3.0) * t7343 * t25114 - F::new(2.0) / F::new(3.0) * t25117 * t2048 - F::new(2.0) / F::new(3.0) * t25120 * t2048 - F::new(4.0) / F::new(3.0) * t6963 * t7352 + t26207;
    (t26207, t26208)
}
