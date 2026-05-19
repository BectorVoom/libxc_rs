//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 833/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk833<F: Float>(t262: F, t41055: F, t2068: F, t2123: F, t551: F, t305: F, t38674: F, t118: F, t25809: F, t39692: F, t5271: F, t558: F) -> (F, F, F, F, F, F, F) {
    let t41056 = t262 * t41055;
    let t41057 = t2068 * t41056;
    let t41059 = t2123 * t551;
    let t41114 = t305 * t38674;
    let t41115 = F::cast_from(0.79828278012425390426e-1_f64) * t41114;
    let t41116 = t118 * t25809;
    let t41120 = t5271 * t39692;
    let t41122 = t2123 * t558;
    (t41056, t41057, t41059, t41115, t41116, t41120, t41122)
}
