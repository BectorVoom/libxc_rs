//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 822/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk822<F: Float>(t8607: F, t8619: F, t8625: F, t7331: F, t7350: F, t7366: F, t8133: F, t8144: F, t8145: F, t8146: F, t8598: F, t8603: F, t8611: F, t8615: F, t8623: F) -> F {
    let t9222 = F::cast_from(0.42874018118069736972e-3_f64) * t8607;
    let t9226 = F::cast_from(0.28015625e-1_f64) * t8619;
    let t9228 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t8625;
    let t9229 = -t8133 + t7331 + F::cast_from(0.18868855373762491241e-2_f64) * t8598 - F::cast_from(0.37737710747524982483e-2_f64) * t8603 + t9222 + F::cast_from(0.21437009059034868486e-2_f64) * t8611 + F::cast_from(0.12862205435420921092e-2_f64) * t8615 + t7350 - F::cast_from(0.31448092289604152069e-3_f64) * t7366 + t8144 - t8145 + t8146 + t9226 - t8623 / F::cast_from(192.0_f64) + t9228;
    t9229
}
