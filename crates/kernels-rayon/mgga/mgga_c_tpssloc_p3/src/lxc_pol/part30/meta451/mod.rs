//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta451 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1713;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1714;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1715;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1716;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta451(t22863: f64, t1995: f64, t9223: f64, t213: f64, t1999: f64, t1338: f64, t6955: f64, t1372: f64, t552: f64, t117: f64, t547: f64, t67: f64, t6559: f64, t225: f64, t794: f64, t6969: f64, t3787: f64, t6604: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22864, t22865, t22868, t22873, t22881, t22891) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1713(t22863, t1995, t9223, t213, t1999, t1338, t6955, t1372, t552, t117, t547, t67);
        let t22892 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1714(t22891, t6559);
        let t22893 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1715(t225, t794);
        let (t22894, t22895, t22897) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1716(t22893, t6969, t22892, t3787, t6604);
    (t22864, t22865, t22868, t22873, t22881, t22891, t22892, t22893, t22894, t22895, t22897)
}
