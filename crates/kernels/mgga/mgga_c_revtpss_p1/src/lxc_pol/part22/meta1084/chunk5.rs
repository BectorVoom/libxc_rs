//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3930/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3930<F: Float>(t2289: F, t5892: F, t21821: F, t625: F, t21824: F, t10208: F, t21829: F, t2339: F, t2340: F, t2366: F, t28036: F, t31035: F, t4287: F, t46144: F, t46146: F, t46148: F, t5915: F, t655: F, t69: F, t75536: F, t75540: F, t75542: F, t75585: F, t75634: F) -> F {
    let t75639 = t2289 * t5892;
    let t75641 = t625 * t21821;
    let t75643 = t625 * t21824;
    let t75655 = t69 * t21829 * t2366 / F::new(4.0) + t69 * t2339 * t75536 / F::new(2.0) - F::new(11.0) / F::new(9.0) * t75540 + F::new(2.0) / F::new(3.0) * t75542 - t69 * t655 * (t75585 + t75634) / F::new(8.0) + F::new(22.0) / F::new(9.0) * t75639 + F::new(4.0) * t75641 - F::new(8.0) / F::new(3.0) * t75643 - F::new(3.0) / F::new(4.0) * t69 * t10208 * t5915 * t2340 - F::new(3.0) * t31035 * t28036 * t4287 + F::new(308.0) / F::new(27.0) * t46144 + F::new(22.0) / F::new(9.0) * t46146 - F::new(11.0) / F::new(9.0) * t46148;
    t75655
}
