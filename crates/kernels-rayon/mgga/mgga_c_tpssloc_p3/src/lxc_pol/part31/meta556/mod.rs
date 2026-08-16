//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1784;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta556(t81598: f64, t81735: f64, t81742: f64, t81849: f64, t81852: f64, t81920: f64, t81954: f64, t2627: f64, t7084: f64, t81688: f64, t81716: f64, t82046: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t84851, t84857, t84859, t84896, t84897, t84921, t84932, t84962, t84995, t85003, t85027) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1784(t81598, t81735, t81742, t81849, t81852, t81920, t81954, t2627, t7084, t81688, t81716, t82046);
    (t84851, t84857, t84859, t84896, t84897, t84921, t84932, t84962, t84995, t85003, t85027)
}
