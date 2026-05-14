//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 696/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk696<F: Float>(t10318: F, t1397: F, t9287: F, t2487: F, t2754: F, t9438: F, t9448: F, t12968: F, t34471: F, t34286: F, t10615: F, t40186: F, t12964: F, t587: F, t589: F, t1429: F, t2365: F, t2366: F, t31747: F) -> (F, F, F, F, F, F, F) {
    let t41914 = t1397 * t10318 * t9287;
    let t41918 = t2487 * t9438 * t9448 * t2754;
    let t41947 = t34471 * t12968;
    let t41949 = t34286 * t12968;
    let t41951 = t10615 * t40186;
    let t41954 = t587 * t589 * t12964;
    let t41958 = t1429 * t2365 * t2366 * t31747;
    (t41914, t41918, t41947, t41949, t41951, t41954, t41958)
}
