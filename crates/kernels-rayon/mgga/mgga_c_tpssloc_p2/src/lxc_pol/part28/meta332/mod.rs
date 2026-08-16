//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta332 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1264;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1265;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta332(t116: f64, t547: f64, t1307: f64, t212: f64, t2586: f64, t535: f64, t9534: f64, t9538: f64, t3792: f64, t3850: f64, t1337: f64, t550: f64, t1338: f64, t3879: f64, t3773: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12225, t12228, t12236, t12240, t12248, t12250) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1264(t116, t547, t1307, t212, t2586, t535, t9534, t9538, t3792, t3850, t1337, t550);
        let (t12259, t12267) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1265(t1338, t3879, t3773, t68);
    (t12225, t12228, t12236, t12240, t12248, t12250, t12259, t12267)
}
