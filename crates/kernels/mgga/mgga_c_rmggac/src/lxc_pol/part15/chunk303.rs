//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 303/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk303<F: Float>(t53: F, t60: F, t1818: F, t196: F, t1794: F, t1797: F, t437: F, t983: F, t1802: F, t1805: F, t441: F, t990: F, zeta_threshold: F) -> (F, F) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t1819 = t196 * t1818;
    let t1827 = piecewise3::<f64>(t54, F::new(0.0), -F::new(2.0) / F::new(9.0) * t983 * t1794 + F::new(2.0) / F::new(3.0) * t437 * t1797);
    let t1833 = piecewise3::<f64>(t61, F::new(0.0), -F::new(2.0) / F::new(9.0) * t990 * t1802 + F::new(2.0) / F::new(3.0) * t441 * t1805);
    let t1835 = t1827 / F::new(2.0) + t1833 / F::new(2.0);
    (t1819, t1835)
}
