//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1358/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1358<F: Float>(t35552: F, t35555: F, t35557: F, t35559: F, t35562: F, t35564: F, t35566: F, t35570: F, t35572: F, t35575: F, t35578: F, t35580: F, t35584: F) -> F {
    let t36405 = F::new(0.36207601172307334926e-6) * t35552 + F::new(0.36207601172307334926e-6) * t35555 - F::new(0.11948508386861420526e-3) * t35557 - F::new(0.75106634031756181752e-5) * t35559 - F::new(0.3090101514449397192e-4) * t35562 - F::new(0.16027743791133485603e-4) * t35564 - F::new(0.809822844183586641e-4) * t35566 + F::new(0.809822844183586641e-4) * t35570 + F::new(0.39141437468873354315e-3) * t35572 - F::new(0.2429468532550759923e-3) * t35575 - F::new(0.12147342662753799615e-3) * t35578 - F::new(0.2429468532550759923e-3) * t35580 + F::new(0.2429468532550759923e-3) * t35584;
    t36405
}
