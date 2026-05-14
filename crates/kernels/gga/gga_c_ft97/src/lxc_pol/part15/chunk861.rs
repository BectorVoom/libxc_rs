//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 861/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk861<F: Float>(t21856: F, t2253: F, t21893: F, t21867: F, t21837: F, t668: F, t1268: F, t4635: F, t21839: F, t21878: F, t8675: F, t21881: F, t21863: F, t21850: F, t21852: F, t21885: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t82088 = t2253 * t21856;
    let t82095 = t2253 * t21893;
    let t82097 = t2253 * t21867;
    let t82112 = t21837 * t668;
    let t82182 = t4635 * t1268;
    let t82267 = t2253 * t21839;
    let t82303 = t8675 * t21878;
    let t82326 = t8675 * t21881;
    let t82328 = t8675 * t21863;
    let t82342 = t21850 * t668;
    let t82361 = t2253 * t21852;
    let t82367 = t8675 * t21885;
    (t82088, t82095, t82097, t82112, t82182, t82267, t82303, t82326, t82328, t82342, t82361, t82367)
}
