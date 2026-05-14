//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1050/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1050<F: Float>(t1010: F, t2928: F, t2938: F, t10410: F, t10413: F, t10417: F, t10433: F, t10436: F, t10440: F, t313: F, t6678: F, t10443: F, t1276: F, t2378: F, t321: F, t6661: F, t819: F, t8358: F, t9640: F) -> (F, F, F, F) {
    let t10451 = t2928 * t1010;
    let t10454 = t1010 * t2938;
    let t10466 = 3.0 / 10.0 * t313 * (-10.0 / 27.0 * t10410 + 10.0 / 3.0 * t10413 + 5.0 / 3.0 * t10417 - 10.0 / 27.0 * t10433 + 10.0 / 3.0 * t10436 + 5.0 / 3.0 * t10440) - t6678;
    let t10468 = -3.0 * t9640 * t1010 + t10443 * t321 - 6.0 * t6661 * t10451 + 6.0 * t1276 * t10454 - t819 * t10466 - 3.0 * t2378 * t2938 + 6.0 * t8358 * t2928;
    (t10451, t10454, t10466, t10468)
}
