//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta605 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1850;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta605(t87931: f64, t10143: f64, t7844: f64, t27143: f64, t532: f64, t90459: f64, t90468: f64, t90470: f64, t90472: f64, t225: f64, t27137: f64, t27059: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92976, t93000, t93286, t93306, t93309, t93310, t93311, t93313, t93316) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1850(t87931, t10143, t7844, t27143, t532, t90459, t90468, t90470, t90472, t225, t27137, t27059);
    (t92976, t93000, t93286, t93306, t93309, t93310, t93311, t93313, t93316)
}
