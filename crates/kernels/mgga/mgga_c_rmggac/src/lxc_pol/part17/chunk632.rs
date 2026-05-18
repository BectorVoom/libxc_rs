//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 632/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk632<F: Float>(t236: F, t8829: F, t3352: F, t7230: F, t333: F, t615: F, t511: F, t1971: F, t352: F, t515: F, t2320: F, t7717: F) -> (F, F, F, F, F, F, F) {
    let t8830 = t236 * t8829;
    let t8831 = t3352 * t8830;
    let t8832 = t7230 * t8831;
    let t8834 = t615 * t333;
    let t8835 = t511 * t8834;
    let t8836 = t1971 * t8835;
    let t8837 = t7230 * t8836;
    let t8842 = t515 * t615 * t352;
    let t8843 = t1971 * t8842;
    let t8844 = t7230 * t8843;
    let t8846 = t7717 * t2320;
    (t8831, t8832, t8836, t8837, t8843, t8844, t8846)
}
