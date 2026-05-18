//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1026/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1026<F: Float>(t108531: F, t6757: F, t35385: F, t6050: F, t30671: F, t2035: F, t35924: F, t709: F, t224: F, t6793: F, t9682: F, t213: F, t665: F) -> (F, F, F, F, F, F) {
    let t150580 = t6757 * t108531;
    let t150590 = t35385 * t6050;
    let t150591 = t30671 * t150590;
    let t150594 = t2035 * t35924 * t709;
    let t150602 = t224 * t9682 * t6793;
    let t150603 = t665 * t213;
    (t150580, t150590, t150591, t150594, t150602, t150603)
}
