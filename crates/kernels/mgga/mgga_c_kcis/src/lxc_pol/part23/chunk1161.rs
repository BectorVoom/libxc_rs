//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1161/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1161<F: Float>(t91794: F, t91796: F, t91799: F, t91801: F, t91804: F, t91806: F, t91809: F, t91811: F, t91814: F, t91816: F, t91818: F, t91820: F, t91822: F, t91825: F) -> F {
    let t92134 = -F::cast_from(0.1125e1_f64) * t91794 - F::cast_from(0.5625e0_f64) * t91796 - F::cast_from(0.1125e1_f64) * t91799 + F::cast_from(0.97125e0_f64) * t91801 - F::cast_from(0.225e1_f64) * t91804 - F::cast_from(0.5625e0_f64) * t91806 + F::cast_from(0.1125e1_f64) * t91809 + F::cast_from(0.1125e1_f64) * t91811 + F::cast_from(0.809375e-1_f64) * t91814 + F::cast_from(0.2428125e0_f64) * t91816 + F::cast_from(0.1125e1_f64) * t91818 - F::cast_from(0.485625e1_f64) * t91820 - F::cast_from(0.3375e1_f64) * t91822 - F::cast_from(0.485625e0_f64) * t91825;
    t92134
}
