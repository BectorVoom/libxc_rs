//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1054/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1054<F: Float>(t145335: F, t22819: F, t7195: F, t115567: F, t136560: F, t136561: F, t136692: F, t136693: F, t136694: F, t136740: F, t136885: F, t136893: F, t136899: F, t136952: F, t136988: F, t145491: F, t22736: F, t25774: F, t34421: F, t399: F, t930: F) -> F {
    let t145536 = t22819 * t7195 * t145335;
    let t145553 = -F::new(0.11854761295685025975e-1) * t34421 * t399 - F::new(0.22705522127871165896e-3) * t145536 + F::new(0.24511020009968991682e-5) * t136692 * t136693 * t136694 * t930 + t136885 - t136893 - F::new(0.90822088511484663584e-3) * t136899 - F::new(0.13200366700519885118e-5) * t136560 * t136561 * t145491 - F::new(0.13200366700519885118e-5) * t136560 * t136561 * t115567 + F::new(0.3827206426927081041e-8) * t22736 * t136740 * t25774 - F::new(0.11738898233082762228e-1) * t136952 - t136988;
    t145553
}
