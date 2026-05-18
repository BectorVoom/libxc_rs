//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1285/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1285<F: Float>(t35588: F, t35591: F, t35595: F, t35597: F, t35599: F, t35601: F, t35606: F, t35610: F, t35613: F, t35615: F, t35618: F, t35620: F, t35623: F) -> F {
    let t37444 = -F::new(0.2429468532550759923e-3) * t35588 + F::new(0.4858937065101519846e-3) * t35591 - F::new(0.128754229768724883e-6) * t35595 + F::new(0.3475929712541504153e-4) * t35597 + F::new(0.21055393948773252666e-2) * t35599 + F::new(0.21055393948773252666e-2) * t35601 - F::new(0.36628003284606854718e-5) * t35606 + F::new(0.6180203028898794384e-4) * t35610 + F::new(0.6180203028898794384e-4) * t35613 + F::new(0.2429468532550759923e-3) * t35615 + F::new(0.3475929712541504153e-4) * t35618 - F::new(0.16221005325193686047e-3) * t35620 + F::new(0.3475929712541504153e-3) * t35623;
    t37444
}
