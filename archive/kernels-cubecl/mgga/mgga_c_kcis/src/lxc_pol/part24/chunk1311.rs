//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1311/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1311<F: Float>(t99865: F, t99867: F, t99869: F, t99871: F, t99874: F, t99876: F, t99878: F, t99880: F, t99882: F, t99884: F, t99886: F, t99888: F, t99890: F, t99892: F, t99894: F, t99896: F, t99898: F, t99901: F) -> F {
    let t101641 = F::cast_from(0.55555555555555555555e-1_f64) * t99865 - F::cast_from(0.33333333333333333333e0_f64) * t99867 + F::cast_from(0.125e0_f64) * t99869 - F::cast_from(0.4046875e-1_f64) * t99871 - F::cast_from(0.809375e-1_f64) * t99874 + F::cast_from(0.1875e0_f64) * t99876 - F::cast_from(0.20234375e-1_f64) * t99878 + F::cast_from(0.89930555555555555557e-2_f64) * t99880 - F::cast_from(0.53958333333333333334e-1_f64) * t99882 + F::cast_from(0.26979166666666666667e-1_f64) * t99884 + F::cast_from(0.25e0_f64) * t99886 + F::cast_from(0.25e0_f64) * t99888 + F::cast_from(0.17986111111111111111e-1_f64) * t99890 - F::cast_from(0.89930555555555555557e-2_f64) * t99892 - F::cast_from(0.20833333333333333333e-1_f64) * t99894 + F::cast_from(0.10791666666666666667e0_f64) * t99896 + F::cast_from(0.12140625e0_f64) * t99898 - F::cast_from(0.9375e-1_f64) * t99901;
    t101641
}
