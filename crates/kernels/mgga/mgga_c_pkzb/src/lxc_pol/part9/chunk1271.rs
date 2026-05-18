//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1271/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1271<F: Float>(t22303: F, t841: F, t834: F, t18451: F, t18454: F, t18457: F, t22230: F, t22234: F, t22236: F, t22262: F, t22265: F, t22269: F, t22273: F, t22277: F, t22281: F, t22284: F, t22287: F, t22290: F, t22294: F, t22297: F) -> (F, F, F) {
    let t22304 = t841 * t22303;
    let t22306 = t834 * t22303;
    let t22308 = F::new(0.82785e0) * t18451 - F::new(0.49671e0) * t18454 - F::new(0.16557e0) * t18457 - F::new(0.93932222222222222223e0) * t22230 + t22234 - F::new(0.905775e0) * t22236 + F::new(0.905775e0) * t22262 - F::new(0.49671e0) * t22265 + F::new(0.248355e0) * t22269 + F::new(0.745065e0) * t22273 + F::new(0.745065e0) * t22277 + F::new(0.248355e0) * t22281 - F::new(0.49671e0) * t22284 - F::new(0.99342e0) * t22287 - F::new(0.73586666666666666667e0) * t22290 + t22294 + t22297 + F::new(0.16504875e0) * t22304 + F::new(0.258925e1) * t22306;
    (t22304, t22306, t22308)
}
