//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1260/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1260<F: Float>(t11188: F, t1587: F, t3634: F, t11192: F, t2906: F, t1504: F, t1803: F, t996: F, t35552: F, t35555: F, t35557: F, t35559: F, t35562: F, t35564: F, t35566: F, t35570: F, t35572: F, t35575: F) -> F {
    let t35578 = t11188 * t3634 * t1587;
    let t35580 = t2906 * t11192;
    let t35584 = t996 * t1803 * t3634 * t1504;
    let t35586 = F::new(0.18103800586153667463e-6) * t35552 + F::new(0.18103800586153667463e-6) * t35555 - F::new(0.59742541934307102628e-4) * t35557 - F::new(0.37553317015878090875e-5) * t35559 - F::new(0.1545050757224698596e-4) * t35562 - F::new(0.80138718955667428014e-5) * t35564 - F::new(0.4049114220917933205e-4) * t35566 + F::new(0.4049114220917933205e-4) * t35570 + F::new(0.19570718734436677157e-3) * t35572 - F::new(0.12147342662753799615e-3) * t35575 - F::new(0.60736713313768998074e-4) * t35578 - F::new(0.12147342662753799615e-3) * t35580 + F::new(0.12147342662753799615e-3) * t35584;
    t35586
}
