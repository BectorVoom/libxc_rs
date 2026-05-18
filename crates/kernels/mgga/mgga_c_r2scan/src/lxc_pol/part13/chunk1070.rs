//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1070/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1070<F: Float>(t120: F, t6517: F, t2225: F, t10734: F, t254: F, t255: F, t6314: F, t6321: F, t1415: F, t2116: F, t5: F, t511: F, t57: F, t7: F) -> (F, F, F) {
    let t37816 = t120 * t6517;
    let t37817 = t37816 * t2225;
    let t37822 = t254 * t10734 * t6314 * t255 * t6321;
    let t37823 = F::new(0.71120679974571020322e0) * t37822;
    let t37833 = t5 * t7 * t1415 * t511 * t57 * t2116;
    (t37817, t37823, t37833)
}
