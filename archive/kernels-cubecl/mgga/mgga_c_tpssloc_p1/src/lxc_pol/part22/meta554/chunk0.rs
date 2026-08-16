//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2054/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2054<F: Float>(t40889: F, t68: F, t852: F, t9971: F, t233: F, t9970: F, t2632: F, t10021: F, t812: F, t841: F, t849: F, t23076: F, t241: F, t67: F) -> (F, F, F, F, F, F, F) {
    let t40890 = t68 * t40889;
    let t40917 = t9971 * t852;
    let t40931 = F::cast_from(1.0_f64) / t9970 / t233;
    let t40933 = t2632 * t2632;
    let t40965 = t812 * t841 * t10021;
    let t40966 = t40965 * t849;
    let t40971 = t241 * t23076 * t67;
    (t40890, t40917, t40931, t40933, t40965, t40966, t40971)
}
