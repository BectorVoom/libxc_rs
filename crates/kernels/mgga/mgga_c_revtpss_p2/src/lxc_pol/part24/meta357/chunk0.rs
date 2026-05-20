//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1225/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1225<F: Float>(t225: F, t24042: F, t385: F, t1695: F, t6350: F, t11121: F, t23964: F, t996: F, t24031: F, t1082: F, t23640: F, t378: F) -> (F, F, F, F, F, F, F) {
    let t24044 = t24042 * t225 * t385;
    let t24047 = t6350 * t1695;
    let t24048 = t11121 * t24047;
    let t24061 = t996 * t23964;
    let t24068 = t996 * t24031;
    let t24075 = t1082 * t23964;
    let t24078 = t378 * t23640;
    (t24044, t24047, t24048, t24061, t24068, t24075, t24078)
}
