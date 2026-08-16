//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 819/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk819<F: Float>(t471: F, t8629: F, t97: F, t4695: F, t4703: F, t4721: F, t4880: F, t4891: F, t4901: F, t4964: F, t4967: F, t6946: F, t6948: F, t6951: F, t8545: F, t8547: F, t8552: F, t8555: F, t8556: F) -> (F, F) {
    let t8631 = t97 * t471 * t8629;
    let t8632 = F::cast_from(3.0_f64) * t8631;
    let t8633 = t4695 + t4880 - t6946 + t8545 - t6948 - t4891 + t4703 + t6951 + t8547 - t8552 + t4901 - t8555 + t4721 - t4964 + t4967 + t8556;
    (t8632, t8633)
}
