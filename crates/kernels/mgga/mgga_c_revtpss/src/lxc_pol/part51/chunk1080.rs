//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1080/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1080<F: Float>(t33983: F, t575: F, t4245: F, t8453: F, t508: F, t1310: F, t33639: F, t1843: F, t32171: F, t5517: F, t8454: F, t32322: F, t7935: F) -> (F, F, F, F, F, F, F) {
    let t125208 = t33983 * t575;
    let t125209 = t4245 * t8453;
    let t125211 = F::new(2.0) * t125209 * t508;
    let t125213 = F::new(2.0) * t33639 * t1310;
    let t125215 = F::new(2.0) * t32171 * t1843;
    let t125217 = F::new(2.0) * t8454 * t5517;
    let t125223 = t32322 * t7935;
    (t125208, t125209, t125211, t125213, t125215, t125217, t125223)
}
