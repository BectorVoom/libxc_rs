//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 896/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk896<F: Float>(t27142: F, t28046: F, t28171: F, t28232: F, t3: F, t2042: F, t5795: F, t1916: F, t7331: F, t7334: F, t1459: F, t7950: F) -> (F, F, F, F, F, F) {
    let t28234 = t27142 + t28046 + t28171 + t28232;
    let t28235 = t3 * t28234;
    let t28246 = param_d * t28234;
    let t28257 = F::new(3.0) * t5795 * t2042;
    let t28259 = F::new(6.0) * t1916 * t7331;
    let t28261 = F::new(3.0) * t1916 * t7334;
    let t28263 = F::new(6.0) * t1459 * t7950;
    (t28235, t28246, t28257, t28259, t28261, t28263)
}
