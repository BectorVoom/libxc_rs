//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1065/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1065<F: Float>(t121846: F, t121851: F, t126122: F, t126134: F, t126136: F, t126141: F, t126145: F, t127641: F, t127642: F, t127643: F, t14495: F, t27183: F, t32434: F, t32463: F, t7398: F, t7759: F, t8649: F, t8650: F) -> (F,) {
    let t127659 = t127641 - t127642 - 0.14456046980341999104e-1 * t127643 + 0.57119737665102352616e0 * t8649 * t8650 * t7398 * t7759 - 0.11423947533020470523e1 * t32463 * t121846 * t14495 + 0.7437465841810202164e-3 * t126122 + t121851 + 0.17347256376410398924e1 * t32434 * t27183 + 0.37645955677973955999e-4 * t126134 - 0.66934509195437693771e-4 * t126136 - 0.29749863367240808656e-2 * t126141 - 0.7437465841810202164e-2 * t126145;
    (t127659,)
}
