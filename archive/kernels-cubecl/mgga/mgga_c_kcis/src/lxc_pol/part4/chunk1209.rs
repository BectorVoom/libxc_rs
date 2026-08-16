//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1209/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1209<F: Float>(t2635: F, t5324: F, t3515: F, t11034: F, t11042: F, t11063: F, t11066: F, t11070: F, t11083: F, t11093: F, t11098: F, t11100: F, t15502: F, t15513: F, t15518: F, t15521: F, t3514: F) -> F {
    let t15524 = t5324 * t2635;
    let t15525 = t3515 * t15524;
    let t15528 = t3514 * t15502 / F::cast_from(144.0_f64) - t11034 / F::cast_from(324.0_f64) - t11042 / F::cast_from(864.0_f64) - t11063 / F::cast_from(1296.0_f64) + t11066 / F::cast_from(1728.0_f64) + t11070 / F::cast_from(1296.0_f64) - t11083 / F::cast_from(864.0_f64) + t11093 + F::cast_from(11.0_f64) / F::cast_from(648.0_f64) * t11098 + t11100 / F::cast_from(162.0_f64) - t3514 * t15513 / F::cast_from(72.0_f64) - t15518 - t3514 * t15521 / F::cast_from(288.0_f64) - t3514 * t15525 / F::cast_from(576.0_f64);
    t15528
}
