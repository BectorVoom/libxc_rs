//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1248/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1248<F: Float>(t35569: F, t35573: F, t35580: F, t35585: F, t35587: F, t35601: F, t35603: F, t35616: F, t37624: F, t37625: F, t37631: F, t37632: F, t40063: F, t40068: F, t40072: F, t40076: F, t40080: F) -> F {
    let t41926 = F::new(0.6289618457920830414e-2) * t35569 + F::new(0.42874018118069736972e-2) * t40063 - F::new(0.6289618457920830414e-2) * t35573 + F::new(0.25158473831683321656e-2) * t35580 - F::new(0.50316947663366643312e-2) * t35585 + F::new(0.17149607247227894789e-2) * t35587 + t37624 + t37625 + F::new(0.11321313224257494745e0) * t35601 + t35603 - F::new(0.17149607247227894789e-2) * t40068 - F::new(0.21437009059034868486e-3) * t40072 - F::new(0.31448092289604152069e-3) * t40076 - F::new(0.15724046144802076034e-2) * t40080 + t37631 + t37632 - F::new(0.31448092289604152069e-2) * t35616;
    t41926
}
