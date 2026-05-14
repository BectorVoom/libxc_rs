//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1238/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1238<F: Float>(t22317: F, t27494: F, t17311: F, t28580: F, t1555: F, t29487: F, t4189: F, t48044: F, t8186: F, t12345: F, t29427: F, t5900: F, t97991: F, t103898: F, t2128: F, t2268: F, t23255: F, t27710: F, t28649: F, t28698: F, t29502: F, t29652: F, t40662: F, t4475: F, t4480: F, t60988: F, t6222: F, t6256: F, t7537: F, t7566: F, t8010: F, t8251: F, t94824: F) -> (F, F, F, F, F, F, F) {
    let t103900 = 2.0 * t27494 * t22317;
    let t103905 = 4.0 * t17311 * t28580;
    let t103909 = 2.0 * t4189 * t29487 * t1555;
    let t103914 = 4.0 * t48044 * t8186;
    let t103917 = 12.0 * t12345 * t29427 * t1555;
    let t103925 = 4.0 * t97991 * t5900;
    let t103930 = 4.0 * t2128 * t28698 * t4480 + 4.0 * t4480 * t6256 * t8251 + 2.0 * t4480 * t7566 * t8010 - t2268 * t60988 - t23255 * t8010 - t27710 * t7566 - 2.0 * t28649 * t6256 - 2.0 * t28698 * t6222 - 6.0 * t29502 * t40662 - t29652 * t4475 + 2.0 * t7537 * t94824 + t103898 - t103900 - t103905 - t103909 - t103914 + t103917 - t103925;
    (t103900, t103905, t103909, t103914, t103917, t103925, t103930)
}
