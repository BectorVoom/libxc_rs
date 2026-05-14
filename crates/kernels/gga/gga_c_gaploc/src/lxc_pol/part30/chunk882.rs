//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 882/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk882<F: Float>(t2441: F, t2877: F, t8072: F, t895: F, t3371: F, t528: F, t1564: F, t3338: F, t475: F, t1445: F, t10152: F, t1457: F, t4752: F, t888: F, t2859: F, t10314: F, t6717: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10334 = 0.35750489951850426669e0 * t2441 * t2877;
    let t10336 = 0.35750489951850426669e0 * t895 * t8072;
    let t10337 = t528 * t3371;
    let t10340 = t1564 * t3338;
    let t10341 = t10340 * t475;
    let t10342 = t1445 * t10341;
    let t10345 = t1457 * t10152;
    let t10348 = t4752 * t888;
    let t10350 = 0.7150097990370085334e0 * t2859 * t10348;
    let t10351 = t6717 * t10314;
    (t10334, t10336, t10337, t10340, t10341, t10342, t10345, t10348, t10350, t10351)
}
