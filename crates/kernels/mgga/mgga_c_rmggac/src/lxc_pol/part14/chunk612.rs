//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 612/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk612<F: Float>(t7828: F, t7883: F, t82: F, t72: F, t2150: F, t504: F, t2127: F, t302: F, t2025: F, t4965: F, t290: F) -> (F, F, F, F, F, F, F, F) {
    let t7884 = t7828 + t7883;
    let t7885 = t82 * t7884;
    let t7886 = t72 * t7885;
    let t7887 = t504 * t2150;
    let t7888 = F::new(0.39914139006212695214e-1) * t7887;
    let t7889 = t302 * t2127;
    let t7890 = t72 * t7889;
    let t7891 = F::new(2.0) * t7890;
    let t7892 = t4965 * t2025;
    let t7893 = F::new(0.79828278012425390428e-1) * t7892;
    let t7894 = t290 * t2127;
    (t7884, t7885, t7886, t7888, t7889, t7891, t7893, t7894)
}
