//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 997/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk997<F: Float>(t12347: F, t11304: F, t11306: F, t11309: F, t11314: F, t11318: F, t11323: F, t11327: F, t11330: F, t11334: F, t11337: F, t11339: F, t11345: F, t11348: F, t11351: F, t11353: F, t11358: F, t11363: F, t11367: F, t11369: F) -> (F, F) {
    let t12348 = F::cast_from(2.0_f64) * t12347;
    let t12368 = F::cast_from(0.3623181683912940217e-6_f64) * t11304 - F::cast_from(0.3623181683912940217e-6_f64) * t11306 + F::cast_from(0.13259557375557346398e-6_f64) * t11309 + F::cast_from(0.69504740211613770836e-3_f64) * t11314 + F::cast_from(0.69504740211613770836e-3_f64) * t11318 - F::cast_from(0.10298285674687440379e-4_f64) * t11323 - F::cast_from(0.14068827330203670243e-7_f64) * t11327 + F::cast_from(0.13259557375557346398e-6_f64) * t11330 - F::cast_from(0.50680539737635041234e-3_f64) * t11334 - F::cast_from(0.50603841145833333338e-5_f64) * t11337 - F::cast_from(0.80966145833333333339e-4_f64) * t11339 + F::cast_from(0.48917046440972222227e-4_f64) * t11345 - F::cast_from(0.69504740211613770836e-3_f64) * t11348 - F::cast_from(0.80966145833333333339e-4_f64) * t11351 + F::cast_from(0.21642471925239962897e-3_f64) * t11353 - F::cast_from(0.84412963981222021456e-7_f64) * t11358 + F::cast_from(0.15716995342493974598e-7_f64) * t11363 - F::cast_from(0.10567613244746075633e-6_f64) * t11367 - F::cast_from(0.26519114751114692796e-6_f64) * t11369;
    (t12348, t12368)
}
