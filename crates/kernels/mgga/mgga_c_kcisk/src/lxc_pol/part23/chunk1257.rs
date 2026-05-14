//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1257/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1257<F: Float>(t382: F, t5967: F, t2152: F, t4143: F, t21001: F, t140: F, t3529: F, t5598: F, t13436: F, t2110: F, t1219: F, t19710: F, t164: F, t398: F, t2168: F, t3988: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t51845 = t382 * t5967;
    let t51854 = t4143 * t2152;
    let t52017 = t21001 * sigma0;
    let t52483 = t140 * t5598 * t3529;
    let t52538 = t2110 * t13436;
    let t52891 = t19710 * t1219;
    let t53214 = t164 * t398;
    let t53303 = t2168 * t3988;
    (t51845, t51854, t52017, t52483, t52538, t52891, t53214, t53303)
}
