//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2100/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2100<F: Float>(t21881: F, t572: F, t7330: F, t1916: F, t28271: F, t28268: F, t1459: F, t30185: F, t5883: F, t6982: F, t25082: F, t86771: F, t8717: F) -> (F, F, F, F, F, F) {
    let t105837 = F::new(6.0) * t572 * t7330 * t21881;
    let t105839 = F::new(12.0) * t1916 * t28271;
    let t105841 = F::new(12.0) * t1916 * t28268;
    let t105843 = F::new(6.0) * t1459 * t30185;
    let t105850 = t6982 * t5883;
    let t105859 = F::new(3.0) * t25082 * t8717 * t86771;
    (t105837, t105839, t105841, t105843, t105850, t105859)
}
