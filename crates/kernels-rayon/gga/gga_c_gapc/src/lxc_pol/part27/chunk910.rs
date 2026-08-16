//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 910/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk910(t11270: f64, t8450: f64, t2923: f64, t5589: f64, t674: f64, t2906: f64, t3635: f64, t11237: f64, t11240: f64, t11244: f64, t11246: f64, t11250: f64, t11252: f64, t11255: f64, t11259: f64, t11263: f64, t11265: f64, t11268: f64) -> (f64, f64) {
    let t11271 = t11270 * t8450;
    let t11273 = t2923 * t674 * t5589;
    let t11274 = t11271 * t11273;
    let t11276 = t2906 * t3635;
    let t11278 = -0.27155700879230501195e-5_f64 * t11237 - 0.772525378612349298e-5_f64 * t11240 + 0.42178273134561804217e-6_f64 * t11244 - 0.26319242435966565832e-3_f64 * t11246 + 0.16094278721090610375e-6_f64 * t11250 - 0.29871270967153551314e-4_f64 * t11252 + 0.43449121406768801912e-4_f64 * t11255 + 0.43449121406768801912e-4_f64 * t11259 - 0.27155700879230501195e-5_f64 * t11263 + 0.11388133746331687139e-4_f64 * t11265 - 0.12653481940368541265e-5_f64 * t11268 - 0.3690598899274157869e-6_f64 * t11274 + 0.60736713313768998074e-4_f64 * t11276;
    (t11273, t11278)
}
