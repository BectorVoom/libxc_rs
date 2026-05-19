//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1066/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1066<F: Float>(t10734: F, t254: F, t255: F, t6314: F, t6321: F, t1415: F, t2116: F, t5: F, t511: F, t57: F, t7: F, t2158: F, t37699: F) -> (F, F, F) {
    let t37822 = t254 * t10734 * t6314 * t255 * t6321;
    let t37823 = F::cast_from(0.71120679974571020322e0_f64) * t37822;
    let t37833 = t5 * t7 * t1415 * t511 * t57 * t2116;
    let t37834 = F::cast_from(0.89443204944342177673e-3_f64) * t37833;
    let t37835 = t37699 * t2158;
    (t37823, t37834, t37835)
}
