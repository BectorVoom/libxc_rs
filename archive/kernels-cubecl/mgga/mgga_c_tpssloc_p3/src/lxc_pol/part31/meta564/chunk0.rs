//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1795/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1795<F: Float>(t236: F, t25093: F, t87229: F, t87230: F, t81764: F, t1512: F, t81807: F, t81824: F, t23041: F, t4236: F, t23040: F, t4166: F) -> (F, F, F, F, F, F) {
    let t87233 = t87229 * t87230 * t236 * t25093;
    let t87237 = F::cast_from(119.0_f64) / F::cast_from(864.0_f64) * t81764;
    let t87243 = t81807 * t1512;
    let t87247 = t81824 * t1512;
    let t87255 = t23041 * t4236;
    let t87261 = t4166 * t23040;
    (t87233, t87237, t87243, t87247, t87255, t87261)
}
