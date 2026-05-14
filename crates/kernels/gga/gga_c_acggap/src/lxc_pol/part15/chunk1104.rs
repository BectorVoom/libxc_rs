//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1104/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1104<F: Float>(t35936: F, t35938: F, t40330: F, t40332: F, t40336: F, t40340: F, t40344: F, t40347: F, t40350: F, t40354: F, t40358: F, t40361: F, t40365: F, t40369: F, t40371: F, t40374: F, t40377: F, t40381: F) -> (F,) {
    let t42066 = 0.40015750243531754507e-2 * t40330 - 0.80031500487063509015e-2 * t40332 - 0.794625e0 * t35936 - 0.52975e0 * t35938 + 0.13753125e0 * t40336 + 0.183375e0 * t40340 - 0.916875e-1 * t40344 - t40347 / 16.0 - t40350 / 8.0 + 0.183375e0 * t40354 - 0.183375e0 * t40358 - 0.916875e-1 * t40361 - 0.916875e-1 * t40365 - 0.916875e-1 * t40369 + 0.3361875e0 * t40371 - 0.183375e0 * t40374 - 0.916875e-1 * t40377 + 0.4584375e0 * t40381;
    (t42066,)
}
