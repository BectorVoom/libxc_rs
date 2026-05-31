//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1396/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1396<F: Float>(t10769: F, t828: F, t1544: F, t836: F, t2746: F, t2710: F, t2713: F, t4371: F, t4353: F, t808: F, t10744: F, t10905: F, t4442: F) -> (F, F, F, F, F, F) {
    let t14785 = t10769 * t828;
    let t14786 = t1544 * t836;
    let t14791 = t2746 * t828;
    let t14817 = t2710 * t2713 * t4371;
    let t14819 = t808 * t4353;
    let t14820 = t10744 * t14819;
    let t14823 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t10905 * t4442;
    (t14785, t14786, t14791, t14817, t14820, t14823)
}
