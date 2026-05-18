//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1219/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1219<F: Float>(t10760: F, t29951: F, t6093: F, t10872: F, t12498: F, t12492: F, t19883: F, t3179: F, t3344: F, t10879: F, t12503: F, t43528: F, t43531: F, t43533: F, t43536: F, t43539: F, t43541: F, t43544: F) -> F {
    let t43547 = t6093 * t10760 * t29951;
    let t43549 = t10872 * t12498;
    let t43551 = t19883 * t12492;
    let t43553 = t3179 * t3344;
    let t43555 = t10879 * t12503;
    let t43557 = -F::new(0.43663693315433241792e-2) * t43528 - F::new(0.43663693315433241792e-2) * t43531 + F::new(0.43663693315433241792e-2) * t43533 - F::new(0.21831846657716620896e-2) * t43536 - F::new(0.65495539973149862688e-2) * t43539 - F::new(0.13099107994629972538e-1) * t43541 - F::new(0.13099107994629972538e-1) * t43544 - F::new(0.13099107994629972538e-1) * t43547 - F::new(0.43341108700271342816e-1) * t43549 - F::new(0.43663693315433241792e-2) * t43551 - F::new(0.23804984598836975486e-2) * t43553 - F::new(0.13002332610081402845e0) * t43555;
    t43557
}
