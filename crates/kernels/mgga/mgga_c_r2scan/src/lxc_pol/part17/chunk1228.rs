//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1228/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1228<F: Float>(t37891: F, t37903: F, t39816: F, t41555: F, t41573: F, t41574: F, t41575: F, t43248: F, t43251: F, t43256: F, t43259: F, t43262: F) -> F {
    let t44308 = F::cast_from(0.2600466522016280569e0_f64) * t43248 + F::cast_from(0.10401866088065122276e1_f64) * t43251 + t41555 - F::cast_from(0.85366933852867742946e0_f64) * t37891 - F::cast_from(0.31147743054556651237e-1_f64) * t37903 + F::cast_from(0.23804984598836975487e0_f64) * t39816 - F::cast_from(0.5200933044032561138e0_f64) * t43256 - F::cast_from(0.34672886960217074252e0_f64) * t43259 - F::cast_from(0.52009330440325611378e0_f64) * t43262 - t41573 - t41574 - t41575;
    t44308
}
