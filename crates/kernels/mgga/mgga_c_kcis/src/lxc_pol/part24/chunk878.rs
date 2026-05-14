//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 878/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk878<F: Float>(t1851: F, t829: F, t4580: F, t3515: F, t4566: F, t11020: F, t1233: F, t18866: F, t18868: F, t18870: F, t18872: F, t18874: F, t18947: F, t18949: F, t18970: F, t18973: F, t18976: F, t18980: F, t18983: F, t18987: F, t18993: F, t19044: F, t4741: F, t4760: F, t5261: F) -> (F, F, F, F, F) {
    let t20344 = t1851 * t829;
    let t20345 = t4580 * t20344;
    let t20346 = t3515 * t20345;
    let t20349 = t4566 * t20344;
    let t20350 = t11020 * t20349;
    let t20361 = t18866 + t18868 + t18870 - t18872 + t18874 + t18947 + t18949 - t18970 - t18973 - t18976 + t18980 + t18983 + t18987 + 0.11696446794910408142e1 * t1233 * t19044 + 0.23392893589820816284e1 * t5261 * t4741 - 0.11696446794910408142e1 * t5261 * t4760 - t18993;
    (t20345, t20346, t20349, t20350, t20361)
}
