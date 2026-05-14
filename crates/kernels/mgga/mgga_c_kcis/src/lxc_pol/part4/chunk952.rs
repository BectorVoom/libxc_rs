//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 952/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk952<F: Float>(t1092: F, t13318: F, t1747: F, t3225: F, t3229: F, t1749: F, t3237: F, t303: F, t4984: F, t922: F, t9517: F, t3200: F, t3178: F, t4985: F, t2825: F, t4814: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t13319 = t1092 * t13318;
    let t13321 = t1747 * t3225;
    let t13322 = t13321 * sigma0;
    let t13323 = t13322 * t3229;
    let t13324 = t1092 * t13323;
    let t13326 = t1749 * t3237;
    let t13327 = t303 * t13326;
    let t13330 = t4984 * t922;
    let t13331 = t9517 * t13330;
    let t13332 = t3200 * t13331;
    let t13336 = t3178 * t4985;
    let t13337 = t1092 * t13336;
    let t13339 = t2825 * t4814;
    (t13319, t13321, t13324, t13327, t13332, t13337, t13339)
}
