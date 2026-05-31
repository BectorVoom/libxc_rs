//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1217/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1217<F: Float>(t3262: F, t3472: F, t43802: F, t12056: F, t3579: F, t495: F, t797: F, t11199: F, t3275: F, t9573: F, t42978: F, t42980: F, t42982: F, t42985: F, t42988: F, t42991: F, t42994: F, t42996: F, t42999: F, t43002: F, t43004: F) -> (F, F, F, F) {
    let t44161 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t3262 * t3472 * t43802;
    let t44165 = t3579 * t495 * t12056 * t797 / F::cast_from(2.0_f64);
    let t44168 = t3275 * t11199 * t9573 / F::cast_from(2.0_f64);
    let t44181 = -F::cast_from(0.46230515946956099003e0_f64) * t42978 + F::cast_from(0.32927245914677557992e0_f64) * t42980 + F::cast_from(0.21951497276451705328e0_f64) * t42982 - F::cast_from(0.17336443480108537126e0_f64) * t42985 - F::cast_from(0.52009330440325611378e0_f64) * t42988 + F::cast_from(0.34672886960217074252e0_f64) * t42991 - F::cast_from(0.10401866088065122276e1_f64) * t42994 - F::cast_from(0.17336443480108537126e0_f64) * t42996 + F::cast_from(0.86682217400542685632e-1_f64) * t42999 + F::cast_from(0.2600466522016280569e0_f64) * t43002 - F::cast_from(0.51220160311720645765e0_f64) * t43004;
    (t44161, t44165, t44168, t44181)
}
