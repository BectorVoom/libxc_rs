//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 390/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk390<F: Float>(t2021: F, t2642: F, t1586: F, t2011: F, t2013: F, t2630: F, t2634: F, t2638: F, t782: F, t788: F, t2029: F, t1994: F, t2033: F, t2475: F, t2511: F, t2530: F, t2535: F, t2539: F, t2618: F, t795: F) -> (F, F, F, F, F) {
    let t2643 = t2021 * t2642;
    let t2644 = t1586 * t2643;
    let t2647 = 0.2698618307426597582e-1 * t2630 * t788 - 0.71963154864709268853e-1 * t2634 * t788 + t2011 + 0.89953943580886586067e-2 * t2013 * t2638 - 0.2698618307426597582e-1 * t782 * t2644;
    let t2648 = t2647 * t2029;
    let t2656 = t2618 * t795 - 0.193e0 * t1994 * t2648 + t2033 + 0.11607361111111111111e-2 * t2475 + 0.17411041666666666666e-2 * t2511 - 0.17411041666666666666e-2 * t2530 - 0.46429444444444444443e-2 * t2535 + 0.11607361111111111111e-2 * t2539;
    (t2643, t2644, t2647, t2648, t2656)
}
