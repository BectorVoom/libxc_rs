//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1718/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1718(t22863: f64, t1995: f64, t9223: f64, t213: f64, t1999: f64, t1338: f64, t6955: f64, t1372: f64, t552: f64, t117: f64, t547: f64, t67: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22864 = 35.0_f64 / 432.0_f64 * t22863;
    let t22865 = t9223 * t1995;
    let t22866 = t22865 * t213;
    let t22867 = t22866 * t1999;
    let t22868 = 0.11304371706359309439e-1_f64 * t22867;
    let t22873 = t1338 * t6955;
    let t22881 = t552 * t1372;
    let t22891 = t547 * t67 * t117;
    (t22864, t22865, t22868, t22873, t22881, t22891)
}
