//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 634/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk634<F: Float>(t1066: F, t779: F, t655: F, t2888: F, t154: F, t2739: F, t742: F, t178: F, t2024: F, t2020: F) -> (F, F, F, F, F, F) {
    let t2889 = t779 * t1066;
    let t2890 = t2889 * t655;
    let t2891 = t2888 * t2890;
    let t2895 = t154 * t742 * t2739;
    let t2898 = t2024 * t178;
    let t2899 = t2020 * t2898;
    (t2889, t2890, t2891, t2895, t2898, t2899)
}
