//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 878/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk878<F: Float>(t150081: F, t2354: F, t6118: F, t684: F, t27805: F, t33341: F, t3746: F, t10157: F, t24437: F, t27814: F, t33319: F, t150036: F, t150040: F, t150044: F, t150047: F, t150051: F, t150054: F, t150058: F, t150062: F, t150066: F, t150069: F, t150073: F, t150077: F, t150079: F) -> (F, F, F, F) {
    let t150084 = t6118 * t2354 * t150081 * t684;
    let t150088 = t27805 * t2354 * t33341 * t3746;
    let t150092 = t24437 * t10157 * t33319 * t27814;
    let t150094 = 2.0 / 9.0 * t150036 - t150040 / 3.0 + t150044 - 2.0 / 3.0 * t150047 - t150051 - 6.0 * t150054 + t150058 - 2.0 / 3.0 * t150062 + 2.0 / 3.0 * t150066 - 2.0 / 9.0 * t150069 + t150073 / 6.0 + t150077 / 6.0 - t150079 / 18.0 + t150084 / 6.0 - t150088 / 3.0 + 3.0 / 2.0 * t150092;
    (t150084, t150088, t150092, t150094)
}
