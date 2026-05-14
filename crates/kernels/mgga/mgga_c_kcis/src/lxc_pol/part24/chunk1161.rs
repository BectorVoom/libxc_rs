//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1161/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1161<F: Float>(t99865: F, t99867: F, t99869: F, t99871: F, t99874: F, t99876: F, t99878: F, t99880: F, t99882: F, t99884: F, t99886: F, t99888: F, t99890: F, t99892: F, t99894: F, t99896: F, t99898: F, t99901: F) -> (F,) {
    let t101641 = 0.55555555555555555555e-1 * t99865 - 0.33333333333333333333e0 * t99867 + 0.125e0 * t99869 - 0.4046875e-1 * t99871 - 0.809375e-1 * t99874 + 0.1875e0 * t99876 - 0.20234375e-1 * t99878 + 0.89930555555555555557e-2 * t99880 - 0.53958333333333333334e-1 * t99882 + 0.26979166666666666667e-1 * t99884 + 0.25e0 * t99886 + 0.25e0 * t99888 + 0.17986111111111111111e-1 * t99890 - 0.89930555555555555557e-2 * t99892 - 0.20833333333333333333e-1 * t99894 + 0.10791666666666666667e0 * t99896 + 0.12140625e0 * t99898 - 0.9375e-1 * t99901;
    (t101641,)
}
