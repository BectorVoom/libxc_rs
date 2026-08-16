//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta612 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2051;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2052;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta612(t2109: f64, t83728: f64, t83737: f64, t24525: f64, t9239: f64, t39063: f64, t7245: f64, t2108: f64, t2240: f64, t2244: f64, t39049: f64, t9231: f64, t24503: f64, t33: f64, t39054: f64, t50: f64, t9300: f64, t1240: f64, t3630: f64, t11588: f64, t2127: f64, t221: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t85473, t85476, t85480, t85501, t85507, t85510, t85514) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2051(t2109, t83728, t83737, t24525, t9239, t39063, t7245, t2108, t2240, t2244, t39049, t9231);
        let (t85524, t85536, t85539, t85628, t85639) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2052(t2240, t24503, t33, t39054, t7245, t50, t9300, t1240, t3630, t11588, t2127, t221);
    (t85473, t85476, t85480, t85501, t85507, t85510, t85514, t85524, t85536, t85539, t85628, t85639)
}
