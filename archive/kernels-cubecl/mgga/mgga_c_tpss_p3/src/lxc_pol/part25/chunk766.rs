//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 766/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk766<F: Float>(t3001: F, t5198: F, t1089: F, t1101: F, t4579: F, t926: F, t3038: F, t4573: F, t3033: F, t1098: F, t1558: F, t1564: F, t3027: F, t3089: F, t4212: F, t4217: F, t4239: F, t4258: F, t4261: F, t4276: F) -> (F, F, F, F, F, F) {
    let t5199 = t5198 * t3001;
    let t5201 = F::cast_from(0.17315859105681463759e2_f64) * t1089 * t5199;
    let t5206 = t1101 * t4579;
    let t5207 = t926 * t5206;
    let t5210 = t3038 * t4573;
    let t5211 = t926 * t5210;
    let t5214 = t3033 * t4573;
    let t5215 = t926 * t5214;
    let t5222 = -t3027 - t4258 * t1564 / F::cast_from(288.0_f64) + t4212 * t1558 / F::cast_from(54.0_f64) - t1098 * t5207 / F::cast_from(288.0_f64) - t1098 * t5211 / F::cast_from(144.0_f64) + t1098 * t5215 / F::cast_from(216.0_f64) - t3089 - t4261 / F::cast_from(432.0_f64) - t4217 / F::cast_from(432.0_f64) - t4276 / F::cast_from(3456.0_f64) + t4239 / F::cast_from(2304.0_f64);
    (t5199, t5201, t5206, t5210, t5214, t5222)
}
