//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2771/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2771<F: Float>(t150: F, t190: F, t50034: F, t40076: F, t40079: F, t40194: F, t40198: F, t50884: F, t50887: F, t50889: F, t50891: F, t50892: F, t50894: F, t50897: F, t50898: F, t50899: F, t50900: F, t50902: F, t50905: F) -> (F, F) {
    let t50907 = t150 * t50034 * t190;
    let t50908 = t50884 + t50887 - t50889 + t50891 + t50892 + t50894 + t50897 + t50898 + t40076 - t40079 + t40194 + t40198 + t50899 - t50900 - t50902 + t50905 + t50907;
    (t50907, t50908)
}
