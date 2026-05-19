//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1292/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1292<F: Float>(t14443: F, t27957: F, t7703: F, t2850: F, t4781: F, t4947: F, t27953: F, t9938: F, t1092: F, t27764: F, t283: F, t9531: F) -> (F, F, F, F, F) {
    let t95781 = t14443 * t27957;
    let t95783 = F::cast_from(0.15445601851851851852e-3_f64) * t7703 * t95781;
    let t95785 = t4947 * t4781 * t2850;
    let t95798 = F::cast_from(0.15445601851851851852e-3_f64) * t7703 * t9938 * t27953;
    let t95802 = t1092 * t9531 * t283 * t27764;
    (t95781, t95783, t95785, t95798, t95802)
}
