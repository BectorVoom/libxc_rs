//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1283/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1283<F: Float>(t35628: F, t35631: F, t35634: F, t35638: F, t35640: F, t35643: F, t35647: F, t35650: F, t35653: F, t35656: F, t35659: F, t35662: F, t35664: F) -> F {
    let t37459 = F::new(0.3475929712541504153e-3) * t35628 + F::new(0.17379648562707520765e-3) * t35631 + F::new(0.17379648562707520765e-3) * t35634 + F::new(0.10862280351692200478e-4) * t35638 + F::new(0.21724560703384400956e-4) * t35640 + F::new(0.128754229768724883e-5) * t35643 - F::new(0.95044953077306754182e-5) * t35647 - F::new(0.95044953077306754182e-5) * t35650 + F::new(0.88482918641258390322e-6) * t35653 + F::new(0.3475929712541504153e-3) * t35656 - F::new(0.3475929712541504153e-3) * t35659 + F::new(0.1619645688367173282e-3) * t35662 + F::new(0.56147717196728673776e-2) * t35664;
    t37459
}
