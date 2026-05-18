//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1296/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1296<F: Float>(t283: F, t3225: F, t359: F, t14073: F, t3200: F, t26692: F, t27803: F, t44575: F, t7703: F, t8037: F, t27772: F, t2911: F, t4781: F) -> (F, F, F, F) {
    let t95848 = t3225 * t283 * t359;
    let t95850 = t3200 * t95848 * t14073;
    let t95852 = t26692 * t27803;
    let t95855 = t7703 * t44575 * t8037;
    let t95860 = t27772 * t4781 * t2911;
    (t95850, t95852, t95855, t95860)
}
