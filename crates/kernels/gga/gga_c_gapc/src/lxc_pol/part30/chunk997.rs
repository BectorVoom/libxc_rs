//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 997/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk997<F: Float>(t11237: F, t11240: F, t11244: F, t11246: F, t11250: F, t11252: F, t11255: F, t11259: F, t11263: F, t11265: F, t11268: F, t11274: F, t11276: F) -> F {
    let t12326 = -F::cast_from(0.10862280351692200478e-4_f64) * t11237 - F::cast_from(0.3090101514449397192e-4_f64) * t11240 + F::cast_from(0.16871309253824721687e-5_f64) * t11244 - F::cast_from(0.10527696974386626333e-2_f64) * t11246 + F::cast_from(0.64377114884362441501e-6_f64) * t11250 - F::cast_from(0.11948508386861420526e-3_f64) * t11252 + F::cast_from(0.17379648562707520765e-3_f64) * t11255 + F::cast_from(0.17379648562707520765e-3_f64) * t11259 - F::cast_from(0.10862280351692200478e-4_f64) * t11263 + F::cast_from(0.45552534985326748555e-4_f64) * t11265 - F::cast_from(0.50613927761474165061e-5_f64) * t11268 - F::cast_from(0.14762395597096631476e-5_f64) * t11274 + F::cast_from(0.2429468532550759923e-3_f64) * t11276;
    t12326
}
