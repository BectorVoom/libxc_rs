//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1122/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1122<F: Float>(t27894: F, t5999: F, t109490: F, t13672: F, t1403: F, t1427: F, t193: F, t2: F, t2354: F, t24186: F, t24204: F, t24208: F, t24245: F, t24257: F, t24423: F, t26: F, t263: F, t27882: F, t27958: F, t27965: F, t28010: F, t28020: F, t3746: F, t4: F, t5996: F, t6008: F, t6745: F, t6844: F, t69154: F, t96820: F) -> (F,) {
    let t109501 = t27894 * t5999 / 9.0;
    let t109515 = 2.0 / 9.0 * t96820 + t69154 * t2 * t4 * t26 * t1427 / 6.0 - t24204 * t28020 / 9.0 + 2.0 / 9.0 * t28010 * t2354 * t24245 * t3746 - 2.0 * t109490 - 2.0 / 3.0 * t1403 * t193 * t27882 * t24423 - t1403 * t193 * t27882 * t24186 / 3.0 - t109501 - t1403 * t193 * t6008 * t263 * t13672 / 3.0 + t24257 * t6844 / 6.0 + t6745 * t24208 / 3.0 - 2.0 / 3.0 * t5996 * t27958 - 2.0 / 3.0 * t5996 * t27965;
    (t109515,)
}
