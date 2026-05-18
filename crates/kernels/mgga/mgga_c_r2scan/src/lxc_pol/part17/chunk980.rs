//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 980/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk980<F: Float>(t12033: F, t3269: F, t11325: F, t3275: F, t3582: F, t1044: F, t3560: F, t11345: F, t3579: F, t11625: F, t3465: F, t11475: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12034 = t3269 * t12033;
    let t12035 = t12034 / F::new(4.0);
    let t12037 = t3275 * t11325 * t3582;
    let t12038 = F::new(5.0) / F::new(16.0) * t12037;
    let t12039 = t3560 * t1044;
    let t12040 = t3579 * t11345;
    let t12041 = t12040 / F::new(4.0);
    let t12042 = t3465 * t11625;
    let t12043 = t3275 * t12042;
    let t12044 = t12043 / F::new(2.0);
    let t12045 = t3465 * t11475;
    (t12034, t12035, t12037, t12038, t12039, t12040, t12041, t12042, t12043, t12044, t12045)
}
