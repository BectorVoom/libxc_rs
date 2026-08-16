//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2976/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2976<F: Float>(t52481: F, t52486: F, t52488: F, t52490: F, t52492: F, t52495: F, t52499: F, t52502: F, t52507: F, t52865: F, t52867: F, t52869: F, t52874: F, t52876: F, t52880: F, t52882: F, t52885: F, t52887: F, t52889: F, t52897: F) -> F {
    let t54233 = t52865 - t52867 - t52869 + t52481 + t52874 - t52876 - t52880 + t52486 + t52882 + t52488 - t52490 + t52492 - t52495 + t52499 + t52502 + t52885 - t52887 - t52889 - t52897 - t52507;
    t54233
}
