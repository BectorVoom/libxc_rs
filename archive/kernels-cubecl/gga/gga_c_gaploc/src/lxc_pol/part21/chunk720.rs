//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 720/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk720<F: Float>(t6706: F, t6717: F, t1411: F, t913: F, t587: F, t2368: F, t4379: F, t2406: F, t540: F, t1508: F, t529: F, t901: F) -> (F, F, F, F, F) {
    let t6718 = t6717 * t6706;
    let t6721 = t1411 * t913;
    let t6722 = t587 * t6721;
    let t6724 = t4379 * t2368;
    let t6726 = t2406 * t540;
    let t6731 = t1508 * t529;
    let t6732 = t6731 * t901;
    (t6718, t6722, t6724, t6726, t6732)
}
