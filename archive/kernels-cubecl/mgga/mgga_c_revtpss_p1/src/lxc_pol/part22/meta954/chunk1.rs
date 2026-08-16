//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3198/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3198<F: Float>(t1774: F, t487: F, t1209: F, t17807: F, t3727: F, t5219: F, t2246: F, t4171: F, t10308: F, t1466: F, t13267: F, t602: F) -> (F, F, F, F, F, F) {
    let t60037 = t487 * t1774;
    let t60087 = t1209 * t17807;
    let t60106 = t5219 * t3727;
    let t60221 = t4171 * t2246;
    let t60224 = t1466 * t10308;
    let t60248 = t13267 * t602;
    (t60037, t60087, t60106, t60221, t60224, t60248)
}
