//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1767/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1767(t22802: f64, t22869: f64, t553: f64, t1338: f64, t6955: f64, t1352: f64, t3851: f64, t6987: f64, t3856: f64, t1372: f64, t552: f64, t1307: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22870 = t22802 + t22869;
    let t22871 = t553 * t22870;
    let t22873 = t1338 * t6955;
    let t22874 = t22873 * t1352;
    let t22877 = t6987 * t3851;
    let t22879 = t6987 * t3856;
    let t22881 = t552 * t1372;
    let t22882 = t22881 * t1307;
    (t22870, t22871, t22873, t22874, t22877, t22879, t22881, t22882)
}
