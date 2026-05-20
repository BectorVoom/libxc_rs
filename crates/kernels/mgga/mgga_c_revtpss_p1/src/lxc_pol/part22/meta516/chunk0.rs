//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2281/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2281<F: Float>(t16840: F, t3436: F, t12238: F, t1733: F, t3379: F, t5105: F, t12327: F, t1723: F, t3391: F, t12331: F, t3390: F, t5079: F) -> (F, F, F, F, F, F, F, F) {
    let t16842 = F::cast_from(0.16081979498692535067e2_f64) * t16840 * t3436;
    let t16844 = F::new(1.0) * t12238 * t1733;
    let t16846 = F::new(2.0) * t3379 * t5105;
    let t16851 = t12327 * t1723;
    let t16852 = t16851 * t3391;
    let t16854 = t12331 * t1723;
    let t16855 = t16854 * t3391;
    let t16857 = t3390 * t5079;
    (t16842, t16844, t16846, t16851, t16852, t16854, t16855, t16857)
}
