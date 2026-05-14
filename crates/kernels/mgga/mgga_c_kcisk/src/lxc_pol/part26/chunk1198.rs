//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1198/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1198<F: Float>(t1459: F, t34888: F, t34852: F, t34854: F, t34856: F, t34859: F, t34861: F, t34864: F, t34867: F, t34870: F, t34872: F, t34874: F, t34876: F, t34878: F, t34880: F, t34882: F, t34884: F, t34886: F) -> (F, F) {
    let t34889 = t1459 * t34888;
    let t34906 = 0.9375e-1 * t34852 - 0.1875e0 * t34854 + 0.125e0 * t34856 + 0.1875e0 * t34859 - 0.125e0 * t34861 - 0.9375e-1 * t34864 - 0.20833333333333333333e-1 * t34867 + 0.625e-1 * t34870 - 0.20234375e-1 * t34872 + 0.4046875e-1 * t34874 - 0.53958333333333333334e-1 * t34876 - 0.4046875e-1 * t34878 + 0.53958333333333333334e-1 * t34880 + 0.20234375e-1 * t34882 - 0.89930555555555555557e-2 * t34884 - 0.26979166666666666667e-1 * t34886;
    (t34889, t34906)
}
