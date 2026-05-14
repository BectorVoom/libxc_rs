//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 521/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk521<F: Float>(t895: F, t898: F, t227: F, t897: F, t224: F, t906: F, t2586: F, t2589: F, t2591: F, t2595: F, t2598: F, t2601: F, t2603: F, t2606: F, t2608: F, t2610: F, t2613: F, t2616: F, t2619: F, t2624: F) -> (F, F, F, F, F) {
    let t2766 = t895 * t898;
    let t2770 = 1.0 / t897 / t227;
    let t2771 = t224 * t2770;
    let t2772 = t906 * t906;
    let t2789 = 0.1875e0 * t2586 - 0.375e0 * t2589 - 0.75e0 * t2591 + 0.375e0 * t2595 + 0.75e0 * t2598 - 0.1875e0 * t2601 + 0.1125e1 * t2603 - 0.4046875e-1 * t2606 + 0.809375e-1 * t2608 + 0.32375e0 * t2610 - 0.809375e-1 * t2613 - 0.32375e0 * t2616 + 0.4046875e-1 * t2619 - 0.809375e0 * t2624;
    (t2766, t2770, t2771, t2772, t2789)
}
