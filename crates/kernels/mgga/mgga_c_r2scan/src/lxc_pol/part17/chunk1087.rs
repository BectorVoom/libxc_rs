//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1087/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1087<F: Float>(t3262: F, t3465: F, t43984: F, t1013: F, t11223: F, t12241: F, t12838: F, t12841: F, t12844: F, t1300: F, t19203: F, t2394: F, t2400: F, t2944: F, t3506: F, t3730: F, t3735: F, t38783: F, t41906: F, t6693: F, t829: F, t9687: F, t9690: F, t9693: F) -> (F, F) {
    let t44579 = 3.0 / 4.0 * t3262 * t3465 * t43984;
    let t44609 = -0.768e1 * t6693 * t3735 * t2394 - 0.1536e2 * t19203 * t12838 * t829 - 0.768e1 * t6693 * t12841 * t829 - 0.384e1 * t6693 * t12844 * t829 - 0.768e1 * t41906 * t2400 - 0.768e1 * t11223 * t9690 - 0.1536e2 * t38783 * t9687 - 0.384e1 * t11223 * t9693 - 0.384e1 * t6693 * t3506 * t2944 - 0.256e1 * t1300 * t12241 * t1013 - 0.256e1 * t1300 * t3730 * t2394;
    (t44579, t44609)
}
