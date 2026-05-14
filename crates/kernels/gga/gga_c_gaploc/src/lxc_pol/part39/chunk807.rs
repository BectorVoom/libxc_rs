//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 807/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk807<F: Float>(t12922: F, t26935: F, t10497: F, t9285: F, t2877: F, t40251: F, t12968: F, t34471: F, t34286: F, t10615: F, t40186: F, t12964: F, t587: F, t589: F, t1429: F, t2365: F, t2366: F, t31747: F) -> (F, F, F, F, F, F, F, F) {
    let t41941 = 0.42900587942220512003e1 * t26935 * t12922;
    let t41942 = t9285 * t10497;
    let t41945 = 0.35750489951850426669e0 * t40251 * t2877;
    let t41947 = t34471 * t12968;
    let t41948 = 0.89376224879626066675e-1 * t41947;
    let t41949 = t34286 * t12968;
    let t41950 = 0.89376224879626066675e-1 * t41949;
    let t41951 = t10615 * t40186;
    let t41952 = 0.89376224879626066675e-1 * t41951;
    let t41954 = t587 * t589 * t12964;
    let t41958 = t1429 * t2365 * t2366 * t31747;
    (t41941, t41942, t41945, t41948, t41950, t41952, t41954, t41958)
}
