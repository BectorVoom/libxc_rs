//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 504/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk504<F: Float>(t740: F, t745: F, t1872: F, t641: F, t79: F, t719: F, t4808: F, t4636: F, t1964: F, t760: F, t755: F, t4722: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5320 = t740 * t745;
    let t5321 = t1872 * t5320;
    let t5322 = t79 * t641;
    let t5330 = F::new(1.0) / t719;
    let t5344 = F::new(0.38691203703703703703e-3) * t4808;
    let t5360 = F::new(0.22831111111111111111e-1) * t4636;
    let t5371 = t1964 * t760;
    let t5372 = F::new(1.0) / t5371;
    let t5373 = t755 * t5372;
    let t5380 = F::new(0.68863333333333333333e0) * t4636;
    let t5387 = F::new(0.17365833333333333333e0) * t4722;
    (t5320, t5321, t5322, t5330, t5344, t5360, t5372, t5373, t5380, t5387)
}
