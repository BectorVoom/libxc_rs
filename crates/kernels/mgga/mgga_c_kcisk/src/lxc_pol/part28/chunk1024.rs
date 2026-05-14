//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1024/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1024<F: Float>(t1737: F, t1746: F, t23632: F, t10937: F, t11105: F, t17379: F, t17382: F, t17402: F, t17635: F, t23460: F, t23463: F, t23466: F, t23469: F, t23472: F, t23475: F, t23478: F, t23481: F, t23484: F, t23487: F, t23490: F) -> (F, F) {
    let t23634 = t1737 * t23632 * t1746;
    let t23654 = -t11105 - 0.79148148148148148147e-2 * t10937 - 0.15829629629629629629e-1 * t17382 + 0.79148148148148148147e-2 * t17402 - t17635 - 0.23744444444444444444e-1 * t17379 + 0.39574074074074074073e-2 * t23460 - 0.19787037037037037037e-1 * t23463 + 0.71233333333333333332e-1 * t23466 + 0.47488888888888888888e-1 * t23469 - 0.11872222222222222222e-1 * t23472 - 0.10685e0 * t23475 - 0.14246666666666666666e0 * t23478 + 0.5936111111111111111e-2 * t23481 - 0.11872222222222222222e-1 * t23484 + 0.35616666666666666666e-1 * t23487 - 0.17808333333333333333e-1 * t23490;
    (t23634, t23654)
}
