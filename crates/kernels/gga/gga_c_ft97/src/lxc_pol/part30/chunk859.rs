//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 859/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk859<F: Float>(t1882: F, t33963: F, t1486: F, t2399: F, t7646: F, t1636: F, t7658: F, t89: F, t33988: F, t375: F, t33860: F, t6308: F, t681: F, t7650: F, t33980: F, t33953: F, t668: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t143329 = t1882 * t33963;
    let t143332 = t1486 * t2399 * t7646;
    let t143333 = 4.0 / 27.0 * t143332;
    let t143335 = t89 * t1636 * t7658;
    let t143336 = 4.0 / 27.0 * t143335;
    let t143339 = t89 * t375 * t33988;
    let t143355 = t6308 * t681 * t33860;
    let t143365 = t1486 * t2399 * t7650;
    let t143366 = 2.0 / 27.0 * t143365;
    let t143371 = t1882 * t33980;
    let t143373 = t33953 * t668;
    (t143329, t143332, t143333, t143335, t143336, t143339, t143355, t143365, t143366, t143371, t143373)
}
