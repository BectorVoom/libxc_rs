//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 605/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk605<F: Float>(t1856: F, t6764: F, t1835: F, t6759: F, t2063: F, t696: F, t173: F, t5089: F, t5122: F, t5125: F, t5128: F, t5129: F, t5130: F, t5137: F, t5142: F, t5150: F, t5158: F, t5168: F, t6667: F) -> (F, F, F, F) {
    let t6891 = t1856 * t6764;
    let t6894 = t1835 * t6759;
    let t6903 = t696 * t2063;
    let t6905 = t5122 - t5125 - t5128 - t5129 - 0.13208333333333333333e-2 * t5158 - 0.117630625e-4 * t5142 + 0.4684e-2 * t5150 - 0.10082625e-4 * t173 * t6891 - 0.672175e-5 * t173 * t6894 - 0.23911438650126355246e-1 * t5089 * t6667 + 0.15538616723388920628e-3 * t5168 * t6667 - 0.11955719325063177623e-1 * t5130 + 0.10359077815592613752e-3 * t5137 - 0.11955719325063177623e-1 * t6903;
    (t6891, t6894, t6903, t6905)
}
