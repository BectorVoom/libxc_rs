//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3881/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3881<F: Float>(t22074: F, t3936: F, t4004: F, t48982: F, t48984: F, t49001: F, t49003: F, t49005: F, t49008: F, t49012: F, t49016: F, t49024: F, t49030: F, t5671: F) -> F {
    let t74574 = -F::cast_from(0.10841600599314203354e-2_f64) * t48982 - F::cast_from(0.80031500487063509015e-2_f64) * t48984 + F::cast_from(0.57165357490759649296e-3_f64) * t49001 - F::cast_from(0.12004725073059526352e-1_f64) * t49003 - F::cast_from(0.80031500487063509015e-2_f64) * t49005 - F::cast_from(0.17149607247227894789e-2_f64) * t5671 * t3936 * t22074 * t4004 - F::cast_from(0.72286371995927450868e-4_f64) * t49008 - F::cast_from(0.4065600224742826258e-4_f64) * t49012 + F::cast_from(0.15246000842785598467e-3_f64) * t49016 - F::cast_from(0.2032800112371413129e-3_f64) * t49024 + F::cast_from(455.0_f64) / F::cast_from(324.0_f64) * t49030;
    t74574
}
