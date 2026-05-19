//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 962/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk962<F: Float>(t32220: F, t32223: F, t1955: F, t2022: F, t1444: F, t25876: F, t545: F, t32216: F) -> (F, F, F, F) {
    let t32225 = F::cast_from(0.25702851531048074406e-1_f64) * t32223 * t32220;
    let t32226 = t1955 * t2022;
    let t32230 = t25876 * t545 * t1444;
    let t32233 = t1955 * t32216;
    (t32225, t32226, t32230, t32233)
}
