//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 462/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk462<F: Float>(t1282: F, t1779: F, t1821: F, t1864: F, t187: F, t1872: F, t437: F, t236: F, t487: F, sigma2: F) -> (F, F, F) {
    let t1876 = t1779 - t1821 + t187 * (-t1282 * t1872 + t1864 * t437 - t1779 + t1821);
    let t1877 = t236 * t1876;
    let t1880 = F::cast_from(1.0_f64) / t487;
    let t1881 = sigma2 * t1880;
    (t1876, t1877, t1881)
}
