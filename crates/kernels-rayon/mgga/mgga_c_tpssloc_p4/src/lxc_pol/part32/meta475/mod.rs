//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta475 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1775;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta475(t5308: f64, t8945: f64, t24995: f64, t1874: f64, t19456: f64, t4028: f64, t6525: f64, t5161: f64, t6996: f64, t1983: f64, t1914: f64, t193: f64, t200: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t24996, t24998, t25005, t25007, t25010, t25011, t25013) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1775(t5308, t8945, t24995, t1874, t19456, t4028, t6525, t5161, t6996, t1983, t1914, t193, t200);
    (t24996, t24998, t25005, t25007, t25010, t25011, t25013)
}
