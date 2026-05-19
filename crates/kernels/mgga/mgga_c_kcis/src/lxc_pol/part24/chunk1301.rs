//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1301/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1301<F: Float>(t100666: F, t100669: F, t100672: F, t100678: F, t101043: F, t101047: F, t27812: F, t93590: F, t96138: F, t96148: F, t96150: F, t96173: F) -> F {
    let t101457 = -t96138 - t93590 + F::cast_from(0.88437037037037037035e-2_f64) * t100666 + F::cast_from(0.66327777777777777776e-2_f64) * t100669 - t96148 - F::cast_from(0.30891203703703703704e-3_f64) * t96150 + F::cast_from(0.16581944444444444444e-2_f64) * t100672 - F::cast_from(0.92673611111111111112e-3_f64) * t96173 - F::cast_from(0.24872916666666666666e-2_f64) * t100678 + F::cast_from(0.185671721767578125e-4_f64) * t27812 * t101043 + F::cast_from(0.111403033060546875e-3_f64) * t27812 * t101047;
    t101457
}
