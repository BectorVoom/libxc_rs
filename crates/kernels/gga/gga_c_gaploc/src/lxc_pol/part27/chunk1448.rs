//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1448/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1448<F: Float>(t1381: F, t3699: F, t1383: F, t12030: F, t501: F, t605: F, t161: F, t39048: F, t12250: F, t13846: F, t1841: F, t1845: F, t1850: F, t1854: F, t1858: F, t29160: F, t29162: F, t29184: F, t29186: F, t29210: F, t29212: F, t29224: F, t29226: F, t29230: F, t39040: F, t39149: F, t5396: F, t7289: F, t734: F) -> (F, F, F) {
    let t39337 = t3699 * t1381;
    let t39339 = F::new(2.0) * t39337 * t1383;
    let t39340 = t12030 * t501;
    let t39342 = F::new(2.0) * t39340 * t605;
    let t39347 = t39048 * t161;
    let t39361 = t29160 - t29162 + t29184 + t29186 - F::new(0.17090058289204942853e-2) * t1841 * t1858 * t13846 * t734 - t29210 - t29212 - t29224 - t29226 + F::new(0.51270174867614828558e-2) * t1841 * t39347 * t1845 - F::new(0.17090058289204942853e-2) * t1850 * t5396 * t39149 - F::new(0.34180116578409885705e-2) * t1841 * t7289 * t39040 + F::new(0.17090058289204942853e-2) * t1850 * t12250 * t161 * t1854 - t29230;
    (t39339, t39342, t39361)
}
