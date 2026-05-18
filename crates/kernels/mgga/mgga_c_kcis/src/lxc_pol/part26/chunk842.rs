//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 842/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk842<F: Float>(t16905: F, t498: F, t12147: F, t5722: F, t1368: F, t5705: F, t3970: F) -> (F, F, F, F) {
    let t16906 = t16905 * t498;
    let t16923 = t12147 * t5722;
    let t16925 = t1368 * t16923 / F::new(432.0);
    let t16933 = t12147 * t5705;
    let t16935 = t1368 * t16933 / F::new(432.0);
    let t16937 = t3970 * t498;
    (t16906, t16925, t16935, t16937)
}
