//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 831/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk831<F: Float>(t355: F, t6680: F, t377: F, t1801: F, t5083: F, t1797: F, t1805: F, t359: F, t6486: F, t376: F, t3464: F, t3338: F, t6491: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t6681 = t6680 * t355;
    let t6682 = t6681 * sigma0;
    let t6683 = t6682 * t377;
    let t6685 = t5083 * t1801;
    let t6687 = t1797 * t1805;
    let t6689 = t359 * t6486;
    let t6690 = t376 * t6689;
    let t6691 = t3464 * t6690;
    let t6693 = t3338 * t6491;
    (t6682, t6683, t6685, t6687, t6689, t6690, t6691, t6693)
}
