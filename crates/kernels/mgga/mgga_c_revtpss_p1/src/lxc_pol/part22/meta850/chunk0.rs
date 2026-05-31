//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2990/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2990<F: Float>(t116: F, t13424: F, t10199: F, t1514: F, t2289: F, t4264: F, t13459: F, t625: F, t13462: F, t13510: F, t105: F, t4283: F, t588: F) -> (F, F, F, F, F, F, F) {
    let t49686 = t13424 * t116;
    let t49698 = t10199 * t1514;
    let t49700 = t2289 * t4264;
    let t49702 = t625 * t13459;
    let t49704 = t625 * t13462;
    let t49724 = t625 * t13510;
    let t49745 = F::cast_from(20.0_f64) * t105 * t4283 * t588;
    (t49686, t49698, t49700, t49702, t49704, t49724, t49745)
}
