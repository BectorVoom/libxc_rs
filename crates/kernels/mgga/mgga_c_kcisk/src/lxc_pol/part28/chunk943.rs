//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 943/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk943<F: Float>(t15851: F, t2537: F, t1869: F, t1894: F, t8780: F, t5203: F, t1873: F, t16588: F, t16596: F, t16598: F, t16614: F, t16616: F, t22347: F, t22351: F, t22353: F, t22355: F) -> (F, F, F, F) {
    let t22357 = t15851 * t2537;
    let t22358 = t1869 * t22357;
    let t22360 = t8780 * t1894;
    let t22361 = t5203 * t22360;
    let t22362 = t1873 * t22361;
    let t22363 = t1869 * t22362;
    let t22365 = 0.22109259259259259259e-2 * t16588 + 0.16581944444444444444e-2 * t22347 - 0.2653111111111111111e-1 * t22351 - t16596 + 0.18424382716049382715e-2 * t22353 + 0.11054629629629629629e-2 * t22355 + 0.33163888888888888888e-2 * t22358 - t16598 + 0.55273148148148148147e-3 * t22363 - t16614 + t16616;
    (t22358, t22360, t22363, t22365)
}
