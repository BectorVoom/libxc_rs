//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2826/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2826<F: Float>(t11773: F, t11865: F, t3205: F, t3206: F, t371: F, t676: F, t2852: F, t3154: F, t2251: F, t1011: F, t3247: F, t697: F) -> (F, F, F, F, F) {
    let t42155 = t11865 * t11773;
    let t42176 = t3205 * t371 * t676 * t3206;
    let t42215 = t3154 * t2852;
    let t42216 = t42215 * t2251;
    let t42254 = t1011 * t697 * t3247;
    (t42155, t42176, t42215, t42216, t42254)
}
