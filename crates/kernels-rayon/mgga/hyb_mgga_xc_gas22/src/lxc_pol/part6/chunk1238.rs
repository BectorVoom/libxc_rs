//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1238/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1238(t2311: f64, t3396: f64, t1363: f64, t6666: f64, t820: f64, t8810: f64, t2272: f64, t3363: f64, t1351: f64, t6709: f64, t1358: f64, t2273: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24848 = t3396 * t2311;
    let t24896 = t1363 * t6666;
    let t24911 = t8810 * t820;
    let t24916 = t3363 * t2272;
    let t24923 = t1351 * t6709;
    let t24926 = t2273 * t1358;
    (t24848, t24896, t24911, t24916, t24923, t24926)
}
