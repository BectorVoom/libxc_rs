//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1302/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1302<F: Float>(t27615: F, t99429: F, t99430: F, t27567: F, t27583: F, t28701: F, t94537: F, t94539: F, t94928: F, t94966: F, t98528: F, t98532: F, t99341: F, t99411: F, t99419: F, t99424: F) -> (F, F) {
    let t99432 = t99429 * t99430 * t27615;
    let t99435 = -F::cast_from(0.23168402777777777778e-3_f64) * t27583 * t99341 + t99411 + F::cast_from(0.46429444444444444443e-2_f64) * t98528 - F::cast_from(0.38691203703703703704e-2_f64) * t98532 + F::cast_from(0.77382407407407407406e-3_f64) * t94537 - F::cast_from(0.51588271604938271604e-3_f64) * t94539 - F::cast_from(0.30945286961263020833e-5_f64) * t94966 * t99419 + t99424 - F::cast_from(0.23168402777777777778e-3_f64) * t27583 * t99419 + F::cast_from(0.23168402777777777778e-3_f64) * t94928 * t28701 + F::cast_from(0.18550940104166666667e-3_f64) * t27567 * t99432;
    (t99432, t99435)
}
