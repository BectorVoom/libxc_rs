//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2916/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2916<F: Float>(t11105: F, t11108: F, t1699: F, t3333: F, t41937: F, t5019: F, t5023: F, t52502: F, t52507: F, t52510: F, t52885: F, t52887: F, t52889: F, t52897: F, t52899: F, t52905: F) -> F {
    let t52906 = -F::new(6.0) * t11105 * t1699 * t41937 * t5023 + F::new(6.0) * t11108 * t3333 * t5019 * t5023 + t52502 - t52507 - t52510 + t52885 - t52887 - t52889 - t52897 + t52899 - t52905;
    t52906
}
