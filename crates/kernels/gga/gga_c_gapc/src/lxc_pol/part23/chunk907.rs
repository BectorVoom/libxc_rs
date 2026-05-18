//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 907/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk907<F: Float>(t11270: F, t8450: F, t2923: F, t5589: F, t674: F, t2906: F, t3635: F, t11237: F, t11240: F, t11244: F, t11246: F, t11250: F, t11252: F, t11255: F, t11259: F, t11263: F, t11265: F, t11268: F) -> (F, F) {
    let t11271 = t11270 * t8450;
    let t11273 = t2923 * t674 * t5589;
    let t11274 = t11271 * t11273;
    let t11276 = t2906 * t3635;
    let t11278 = -F::new(0.27155700879230501195e-5) * t11237 - F::new(0.772525378612349298e-5) * t11240 + F::new(0.42178273134561804217e-6) * t11244 - F::new(0.26319242435966565832e-3) * t11246 + F::new(0.16094278721090610375e-6) * t11250 - F::new(0.29871270967153551314e-4) * t11252 + F::new(0.43449121406768801912e-4) * t11255 + F::new(0.43449121406768801912e-4) * t11259 - F::new(0.27155700879230501195e-5) * t11263 + F::new(0.11388133746331687139e-4) * t11265 - F::new(0.12653481940368541265e-5) * t11268 - F::new(0.3690598899274157869e-6) * t11274 + F::new(0.60736713313768998074e-4) * t11276;
    (t11273, t11278)
}
