//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 636/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk636(t11742: f64, t11775: f64, t11811: f64, t11829: f64, t11866: f64, t11904: f64, t11935: f64, t11966: f64, t3689: f64, t555: f64) -> (f64, f64) {
    let t11969 = t11742 + t11775 + t11811 + t11829 + t11866 + t11904 + t11935 + t11966;
    let t11977 = t555 * t3689;
    (t11969, t11977)
}
