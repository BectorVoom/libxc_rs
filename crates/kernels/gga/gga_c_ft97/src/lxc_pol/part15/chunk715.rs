//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 715/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk715<F: Float>(t86: F, t112: F, t113: F, t20479: F, t20489: F, t4628: F, t4635: F, t5: F, t989: F, t992: F, t4417: F, t8766: F, t8774: F) -> (F, F, F) {
    let t87 = F::new(10000000.0) <= t86;
    let t20494 = piecewise3::<f64>(t87, F::new(0.0), t5 * t20479 * t113 / F::new(4.0) + F::new(3.0) / F::new(4.0) * t5 * t4628 * t992 + F::new(3.0) / F::new(4.0) * t5 * t989 * t4635 + t5 * t112 * t20489 / F::new(4.0));
    let t20507 = t8766 * t4417;
    let t20514 = t8774 * t4417;
    (t20494, t20507, t20514)
}
