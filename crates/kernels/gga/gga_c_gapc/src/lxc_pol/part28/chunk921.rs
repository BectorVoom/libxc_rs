//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 921/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk921<F: Float>(t11225: F, t11229: F, t11231: F, t11183: F, t11186: F, t12012: F, t12013: F, t12014: F, t12015: F, t12016: F, t12017: F, t12018: F, t12019: F, t11240: F, t11244: F, t11246: F) -> (F, F, F, F) {
    let t12020 = 0.86898242813537603825e-4 * t11225;
    let t12021 = 0.2530696388073708253e-5 * t11229;
    let t12022 = 0.3475929712541504153e-3 * t11231;
    let t12023 = 0.54311401758461002391e-5 * t11183 + 0.54311401758461002391e-5 * t11186 - t12012 - t12013 - t12014 + t12015 + t12016 - t12017 + t12018 - t12019 - t12020 + t12021 + t12022;
    let t12025 = 0.1545050757224698596e-4 * t11240;
    let t12026 = 0.84356546269123608433e-6 * t11244;
    let t12027 = 0.52638484871933131665e-3 * t11246;
    (t12023, t12025, t12026, t12027)
}
