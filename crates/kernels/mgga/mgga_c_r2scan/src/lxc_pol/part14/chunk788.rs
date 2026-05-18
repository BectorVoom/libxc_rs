//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 788/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk788<F: Float>(t166: F, t6880: F, t2068: F, t2271: F, t2320: F, t58: F, t766: F, t2330: F, t2333: F, t2332: F, t287: F) -> (F, F, F, F, F, F) {
    let t6881 = t6880 * t166;
    let t6885 = t2271 * t2068;
    let t6887 = t2320 * t58;
    let t6888 = t6887 * t766;
    let t6890 = t2330 * t2333;
    let t6897 = F::new(1.0) / t2332 / t287;
    (t6881, t6885, t6887, t6888, t6890, t6897)
}
