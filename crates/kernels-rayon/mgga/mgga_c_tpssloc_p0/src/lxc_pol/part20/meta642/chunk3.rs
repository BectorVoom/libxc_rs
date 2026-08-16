//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2353/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2353(t10186: f64, t13780: f64, t13785: f64, t13839: f64, t2986: f64, t42837: f64, t10236: f64, t12652: f64, t10913: f64, t13554: f64, t13536: f64, t12648: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48242 = t10186 * t13780;
    let t48244 = t10186 * t13785;
    let t48250 = t2986 * t42837 * t13839;
    let t48256 = t10236 * t12652;
    let t48260 = t13554 * t10913;
    let t48265 = t13536 * t10913;
    let t48269 = t10236 * t12648;
    (t48242, t48244, t48250, t48256, t48260, t48265, t48269)
}
