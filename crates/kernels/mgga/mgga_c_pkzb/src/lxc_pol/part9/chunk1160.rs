//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1160/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1160<F: Float>(t1878: F, t218: F, t3061: F, t3065: F, t22233: F, t18427: F, t18430: F, t18433: F, t18468: F, t22230: F, t22236: F, t22262: F, t841: F, t834: F, t18451: F, t18454: F, t18457: F, t22234: F, t22265: F, t22269: F, t22273: F, t22277: F, t22281: F, t22284: F, t22287: F, t22290: F) -> (F, F, F, F, F) {
    let t22293 = t218 * t1878 * t3061;
    let t22294 = 0.82785e0 * t22293;
    let t22296 = t218 * t1878 * t3065;
    let t22297 = 0.82785e0 * t22296;
    let t22302 = 4.0 / 3.0 * t22233;
    let t22303 = t18468 - 28.0 / 9.0 * t18427 + 4.0 / 3.0 * t18430 - t18433 / 3.0 - 28.0 / 27.0 * t22230 + t22302 - t22236 + t22262;
    let t22304 = t841 * t22303;
    let t22306 = t834 * t22303;
    let t22308 = 0.82785e0 * t18451 - 0.49671e0 * t18454 - 0.16557e0 * t18457 - 0.93932222222222222223e0 * t22230 + t22234 - 0.905775e0 * t22236 + 0.905775e0 * t22262 - 0.49671e0 * t22265 + 0.248355e0 * t22269 + 0.745065e0 * t22273 + 0.745065e0 * t22277 + 0.248355e0 * t22281 - 0.49671e0 * t22284 - 0.99342e0 * t22287 - 0.73586666666666666667e0 * t22290 + t22294 + t22297 + 0.16504875e0 * t22304 + 0.258925e1 * t22306;
    (t22293, t22296, t22304, t22306, t22308)
}
