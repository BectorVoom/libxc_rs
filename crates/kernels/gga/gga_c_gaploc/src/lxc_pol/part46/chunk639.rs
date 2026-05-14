//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 639/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk639<F: Float>(t13069: F, t2684: F, t3247: F, t900: F, t10867: F, t12653: F, t12661: F, t12670: F, t13050: F, t13054: F, t13057: F, t13060: F, t13061: F, t13062: F, t13066: F, t10924: F, t787: F) -> (F, F, F) {
    let t13070 = t2684 * t13069;
    let t13072 = t900 * t3247;
    let t13073 = t10867 * t13072;
    let t13074 = 0.89376224879626066675e-1 * t13073;
    let t13075 = 0.38342925953920749676e0 * t12653 - t13050 - 0.76685851907841499352e0 * t12661 + t13054 - t13057 - t13060 + t13061 - t13062 - 0.38342925953920749676e0 * t13066 - 0.38342925953920749676e0 * t12670 + 0.38342925953920749676e0 * t13070 - t13074;
    let t13077 = t787 * t10924;
    (t13072, t13075, t13077)
}
