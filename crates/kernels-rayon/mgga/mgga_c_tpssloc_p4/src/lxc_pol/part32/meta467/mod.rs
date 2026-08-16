//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta467 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1757;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta467(t24658: f64, t7325: f64, t3030: f64, t3502: f64, t478: f64, t1209: f64, t1222: f64, t7334: f64, t2141: f64, t3540: f64, t3: f64, t7324: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24659, t24660, t24661, t24667, t24668, t24675, t24681, t24682) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1757(t24658, t7325, t3030, t3502, t478, t1209, t1222, t7334, t2141, t3540, t3, t7324);
    (t24659, t24660, t24661, t24667, t24668, t24675, t24681, t24682)
}
