//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta236 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1030;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1031;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1032;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta236(t1385: f64, t1842: f64, t3887: f64, t3787: f64, t68: f64, t544: f64, t1824: f64, t562: f64, t5250: f64, t1825: f64, t3901: f64, t1380: f64, t5287: f64, t1338: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5325, t5326, t5333, t5334) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1030(t1385, t1842, t3887, t3787, t68, t544);
        let t5335 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1031(t1824, t562);
        let (t5336, t5339, t5341, t5343, t5344) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1032(t5250, t5335, t1825, t3901, t1380, t5287, t1338, t68, t544);
    (t5325, t5326, t5333, t5334, t5335, t5336, t5339, t5341, t5343, t5344)
}
