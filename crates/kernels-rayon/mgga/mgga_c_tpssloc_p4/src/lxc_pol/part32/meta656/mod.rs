//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta656 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2085;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta656(t86916: f64, t86955: f64, t86991: f64, t87068: f64, t87080: f64, t87140: f64, t87155: f64, t87177: f64, t87243: f64, t87304: f64, t87345: f64, t87403: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92406, t92432, t92458, t92492, t92497, t92513, t92516, t92543, t92597, t92633, t92652, t92676) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2085(t86916, t86955, t86991, t87068, t87080, t87140, t87155, t87177, t87243, t87304, t87345, t87403);
    (t92406, t92432, t92458, t92492, t92497, t92513, t92516, t92543, t92597, t92633, t92652, t92676)
}
