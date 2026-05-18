//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 775/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk775<F: Float>(t3800: F, t498: F, t1207: F, t1248: F, t3153: F, t3618: F, t828: F, t1209: F, t3781: F, t126: F, t482: F) -> (F, F, F, F, F, F, F) {
    let t12587 = F::new(1.0) / t3800 / t498;
    let t12625 = t1207 * t1207;
    let t12626 = F::new(1.0) / t12625;
    let t12712 = t1248 * t3153;
    let t12787 = t828 * t3618;
    let t12808 = t1209 * t3781;
    let t12915 = t126 * t482;
    (t12587, t12625, t12626, t12712, t12787, t12808, t12915)
}
