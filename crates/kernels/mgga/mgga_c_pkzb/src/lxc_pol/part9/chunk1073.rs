//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1073/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1073<F: Float>(t501: F, t7028: F, t496: F, t4874: F, t7046: F, t4877: F, t2609: F, t5331: F, t1667: F, t6801: F, t5336: F, t16935: F, t16940: F, t1542: F, t2607: F, t16810: F, t16813: F, t16822: F, t16825: F, t16938: F, t16946: F, t16950: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t20362 = t501 * t7028;
    let t20363 = 24.0 * t20362;
    let t20365 = 24.0 * t496 * t7028;
    let t20366 = t7046 * t4874;
    let t20367 = 0.21687162600603479684e-1 * t20366;
    let t20368 = t7046 * t4877;
    let t20369 = 0.32530743900905219526e-1 * t20368;
    let t20370 = t2609 * t5331;
    let t20371 = 0.35089341735807877242e1 * t20370;
    let t20372 = t6801 * t1667;
    let t20373 = 0.73245789224026180216e-3 * t20372;
    let t20374 = t2609 * t5336;
    let t20375 = 0.51947577317044391277e2 * t20374;
    let t20376 = 0.17090684152272775383e-2 * t16935;
    let t20377 = 192.0 * t16940;
    let t20378 = t1542 * t2607;
    let t20379 = 60.0 * t20378;
    let t20380 = -t20363 + t16810 - t16813 - t16822 + t20365 - t20367 - t20369 + t20371 + t20373 - t20375 + t16825 - t20376 + t16938 + t20377 + t16946 + t16950 + t20379;
    (t20363, t20365, t20367, t20369, t20371, t20373, t20375, t20376, t20377, t20379, t20380)
}
