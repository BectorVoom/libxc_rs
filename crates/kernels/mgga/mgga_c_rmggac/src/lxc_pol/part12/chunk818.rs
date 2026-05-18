//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 818/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk818<F: Float>(t1212: F, t1970: F, t1971: F, t209: F, t515: F, t570: F, t7244: F, t8447: F, t321: F, t14243: F, t16503: F, t333: F, t8440: F) -> (F, F, F, F) {
    let t38412 = t1970 * t1971 * t515 * t570 * t1212 * t209;
    let t38414 = t7244 * t8447;
    let t38415 = F::new(0.19863479950205658386e-4) * t38414;
    let t38416 = t209 * t321;
    let t38420 = t16503 * t14243 * t8440 * t38416 * t333;
    (t38412, t38415, t38416, t38420)
}
