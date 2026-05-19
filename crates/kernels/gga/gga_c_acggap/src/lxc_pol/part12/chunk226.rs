//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 226/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk226<F: Float>(t43: F, t50: F, t292: F, t817: F, t818: F, t824: F, t53: F, t238: F, t296: F, zeta_threshold: F) -> (F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t828 = piecewise3::<F>(t44, F::new(0.0), -F::new(2.0) / F::new(9.0) * t817 * t818 + F::new(2.0) / F::new(3.0) * t292 * t824);
    let t829 = F::new(1.0) / t53;
    let t830 = t238 * t238;
    let t833 = -t824;
    let t837 = piecewise3::<F>(t51, F::new(0.0), -F::new(2.0) / F::new(9.0) * t829 * t830 + F::new(2.0) / F::new(3.0) * t296 * t833);
    let t839 = t828 / F::new(2.0) + t837 / F::new(2.0);
    (t829, t830, t833, t839)
}
