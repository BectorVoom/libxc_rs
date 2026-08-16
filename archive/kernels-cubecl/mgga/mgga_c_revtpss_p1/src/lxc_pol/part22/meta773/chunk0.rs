//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2859/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2859<F: Float>(t3367: F, t3603: F, t2251: F, t12839: F, t2258: F, t3555: F, t3766: F, t5330: F, t1209: F, t13147: F, t17708: F, t12854: F, t17350: F) -> (F, F, F, F, F, F) {
    let t44458 = t3603 * t3367;
    let t44459 = t44458 * t2251;
    let t44466 = t12839 * t2258;
    let t44484 = t3555 * t3766 * t5330;
    let t44500 = t1209 * t13147 * t17708;
    let t44510 = t12854 * t17350;
    (t44458, t44459, t44466, t44484, t44500, t44510)
}
