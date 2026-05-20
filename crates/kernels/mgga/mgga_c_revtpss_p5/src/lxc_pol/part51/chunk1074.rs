//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1074/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1074<F: Float>(t121239: F, t32268: F, t121133: F, t25900: F, t1385: F, t240: F, t27: F, t119967: F, t121204: F, t13847: F, t1399: F, t121086: F, t32710: F) -> (F, F, F, F, F, F, F, F) {
    let t121240 = t32268 * t121239;
    let t121241 = t121133 * t25900;
    let t121242 = t121240 * t121241;
    let t121245 = t1385 * t27 * t240;
    let t121246 = t119967 * t121245;
    let t121248 = t13847 * t121204 * t1399;
    let t121249 = t121246 * t121248;
    let t121251 = t32710 * t121086;
    (t121240, t121241, t121242, t121245, t121246, t121248, t121249, t121251)
}
