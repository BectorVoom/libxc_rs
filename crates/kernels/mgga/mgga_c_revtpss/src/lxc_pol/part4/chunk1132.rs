//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1132/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1132<F: Float>(t13937: F, t13943: F, t13946: F, t13949: F, t13954: F, t13956: F, t13959: F, t13962: F, t3934: F, t9796: F, t9799: F, t9804: F, t9822: F) -> F {
    let t13965 = -F::new(0.90357964994909313582e-5) * t9796 - F::new(0.36143185997963725432e-4) * t9799 - F::new(0.21437009059034868486e-3) * t3934 * t13937 + t13943 - F::new(0.42874018118069736972e-3) * t3934 * t13946 - F::new(0.30488190661738479625e-3) * t13949 + t13954 + F::new(0.25410001404642664112e-5) * t13956 + t9804 + F::new(0.10164000561857065645e-3) * t9822 - F::new(0.56688979511669985553e-2) * t13959 + F::new(0.17149607247227894789e-2) * t3934 * t13962;
    t13965
}
