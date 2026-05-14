//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 881/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk881<F: Float>(t3873: F, t687: F, t1616: F, t11304: F, t11306: F, t11309: F, t11314: F, t11318: F, t11323: F, t11327: F, t11330: F, t11334: F, t11337: F, t11339: F, t11345: F, t11348: F, t11351: F, t11353: F, t11358: F, t11363: F, t11367: F, t11369: F) -> (F, F, F, F) {
    let t12346 = t3873 * t687;
    let t12347 = t1616 * t12346;
    let t12348 = 2.0 * t12347;
    let t12368 = 0.3623181683912940217e-6 * t11304 - 0.3623181683912940217e-6 * t11306 + 0.13259557375557346398e-6 * t11309 + 0.69504740211613770836e-3 * t11314 + 0.69504740211613770836e-3 * t11318 - 0.10298285674687440379e-4 * t11323 - 0.14068827330203670243e-7 * t11327 + 0.13259557375557346398e-6 * t11330 - 0.50680539737635041234e-3 * t11334 - 0.50603841145833333338e-5 * t11337 - 0.80966145833333333339e-4 * t11339 + 0.48917046440972222227e-4 * t11345 - 0.69504740211613770836e-3 * t11348 - 0.80966145833333333339e-4 * t11351 + 0.21642471925239962897e-3 * t11353 - 0.84412963981222021456e-7 * t11358 + 0.15716995342493974598e-7 * t11363 - 0.10567613244746075633e-6 * t11367 - 0.26519114751114692796e-6 * t11369;
    (t12346, t12347, t12348, t12368)
}
