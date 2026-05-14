//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1127/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1127<F: Float>(t24237: F, t27993: F, t24564: F, t3977: F, t13830: F, t6187: F, t1403: F, t27975: F, t681: F, t10157: F, t1091: F, t1173: F, t193: F, t2354: F, t2404: F, t24184: F, t24204: F, t24217: F, t24245: F, t24396: F, t28032: F, t28038: F, t28039: F, t3837: F, t6002: F, t6062: F, t6745: F, t683: F, t98143: F, t98166: F, t98172: F) -> (F, F, F) {
    let t109670 = t24237 * t27993 / 27.0;
    let t109671 = t3977 * t24564;
    let t109681 = t13830 * t6187;
    let t109700 = 2.0 / 9.0 * t1403 * t681 * t27975;
    let t109704 = t6745 * t24184 + t109670 - 2.0 * t109671 + 2.0 / 9.0 * t6002 * t683 * t6062 * t28032 - 2.0 / 27.0 * t6002 * t2404 * t6062 * t28038 - 4.0 * t109681 + 2.0 / 9.0 * t98166 + t6745 * t24217 / 6.0 - t6002 * t2354 * t98143 * t1091 / 18.0 + 2.0 * t6002 * t10157 * t24245 * t3837 + t1403 * t193 * t24396 * t1173 / 6.0 + t109700 - 2.0 / 27.0 * t98172 - 2.0 / 27.0 * t24204 * t28039;
    (t109671, t109681, t109704)
}
