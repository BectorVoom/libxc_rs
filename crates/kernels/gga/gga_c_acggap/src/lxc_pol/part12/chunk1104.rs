//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1104/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1104<F: Float>(t31276: F, t8875: F, t1579: F, t2095: F, t355: F, t171: F, t5011: F, t31443: F, t35296: F, t1017: F, t2030: F, t2297: F, t8927: F) -> (F, F, F, F) {
    let t35643 = t31276 * t8875;
    let t35646 = t2095 * t1579 * t355;
    let t35649 = t171 * t5011;
    let t35651 = t31443 * t35649 * t35296;
    let t35656 = t2030 * t8927 * t2297 * t1017;
    (t35643, t35646, t35651, t35656)
}
