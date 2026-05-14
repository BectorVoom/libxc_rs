//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1367/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1367<F: Float>(t119833: F, t119835: F, t119837: F, t119839: F, t119841: F, t119843: F, t119845: F, t119847: F, t119849: F, t119852: F, t119854: F, t119856: F, t119858: F, t119860: F, t119862: F, t119864: F, t119866: F, t119868: F, t119870: F) -> (F,) {
    let t119986 = 0.53958333333333333334e-1 * t119833 + 0.10791666666666666667e0 * t119835 - 0.9375e-1 * t119837 + 0.1875e0 * t119839 - 0.5e0 * t119841 - 0.125e0 * t119843 + 0.27777777777777777777e-1 * t119845 + 0.5e0 * t119847 - 0.4046875e-1 * t119849 - 0.9375e-1 * t119852 + 0.20234375e-1 * t119854 + 0.375e0 * t119856 + 0.21583333333333333333e0 * t119858 + 0.12140625e0 * t119860 + 0.20833333333333333333e-1 * t119862 + 0.1875e0 * t119864 - 0.26979166666666666667e-1 * t119866 + 0.4046875e-1 * t119868 - 0.21583333333333333333e0 * t119870;
    (t119986,)
}
