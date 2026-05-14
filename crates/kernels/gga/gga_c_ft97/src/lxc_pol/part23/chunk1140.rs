//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1140/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1140<F: Float>(t27889: F, t761: F, t107996: F, t107998: F, t108000: F, t108002: F, t108060: F, t108072: F, t108077: F, t108080: F, t108083: F, t108114: F, t108138: F, t108157: F, t108160: F, t108171: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t110024 = t761 * t27889;
    let t110041 = 2.0 / 27.0 * t107996;
    let t110042 = 2.0 / 27.0 * t107998;
    let t110043 = 2.0 / 81.0 * t108000;
    let t110044 = t108002 / 54.0;
    let t110060 = t108060 / 54.0;
    let t110064 = 2.0 / 27.0 * t108072;
    let t110067 = 2.0 / 9.0 * t108077;
    let t110068 = t108080 / 18.0;
    let t110069 = 2.0 / 9.0 * t108083;
    let t110077 = 2.0 / 27.0 * t108114;
    let t110085 = t108138 / 27.0;
    let t110089 = t108157 / 18.0;
    let t110090 = t108160 / 18.0;
    let t110095 = 2.0 / 9.0 * t108171;
    (t110024, t110041, t110042, t110043, t110044, t110060, t110064, t110067, t110068, t110069, t110077, t110085, t110089, t110090, t110095)
}
