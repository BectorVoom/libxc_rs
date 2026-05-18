//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 492/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk492<F: Float>(t213: F, t2551: F, t2653: F, t2740: F, t2820: F, t2014: F, t978: F, t211: F, t215: F, t414: F, t690: F, t2026: F, t982: F, zeta_threshold: F) -> (F, F, F) {
    let t214 = t213 <= zeta_threshold;
    let t2822 = t2551 + t2653 + t2740 + t2820;
    let t2828 = t2014 * t978;
    let t2831 = t215 * t211;
    let t2835 = piecewise3::<f64>(t214, F::new(0.0), F::new(4.0) / F::new(9.0) * t2828 * t690 + F::new(8.0) / F::new(3.0) * t2831 * t414);
    let t2836 = t2026 * t982;
    (t2822, t2835, t2836)
}
