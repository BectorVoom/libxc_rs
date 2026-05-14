//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 812/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk812<F: Float>(t10230: F, t10231: F, t3235: F, t3250: F, t1004: F, t2152: F, t827: F, t1063: F, t10201: F, t10205: F, t10208: F, t10213: F, t10217: F, t10220: F, t10223: F, t10227: F) -> (F, F, F, F, F) {
    let t10232 = t10230 * t10231;
    let t10234 = t3235 * t3250;
    let t10236 = t1004 * t2152;
    let t10237 = t10236 * t827;
    let t10238 = t10237 * t1063;
    let t10240 = -0.74372214241464483348e-4 * t10201 + 0.11742981196020707897e-4 * t10205 + 0.58714905980103539485e-5 * t10208 + 0.56366309740899397906e-3 * t10213 - 0.33406432906439709826e-4 * t10217 - 0.58714905980103539485e-5 * t10220 - 0.342503618217270647e-5 * t10223 - 0.342503618217270647e-5 * t10227 - 0.20299047773010240345e-6 * t10232 - 0.11742981196020707897e-4 * t10234 - 0.58714905980103539485e-5 * t10238;
    (t10232, t10234, t10237, t10238, t10240)
}
