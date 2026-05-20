//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3920/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3920<F: Float>(t1448: F, t5778: F, t13625: F, t13674: F, t21937: F, t22483: F, t3889: F, t4139: F, t47084: F, t49582: F, t5541: F, t5542: F, t74114: F, t74115: F, t74116: F, t74117: F, t74119: F, t74120: F) -> F {
    let t75365 = t5778 * t1448;
    let t75372 = -F::new(6.0) * t13625 * t22483 * t4139 + F::new(8.0) * t13674 * t5541 * t75365 + F::new(3.0) * t21937 * t3889 * t4139 - F::new(6.0) * t4139 * t49582 * t5542 - t47084 - t74114 + t74115 + t74116 - t74117 + t74119 - t74120;
    t75372
}
