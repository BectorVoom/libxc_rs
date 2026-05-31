//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1218/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1218<F: Float>(t19908: F, t28024: F, t26938: F, t29059: F, t1196: F, t18463: F, t19882: F, t95391: F, t20155: F, t283: F, t7749: F, t99865: F, t99867: F, t99869: F, t99871: F, t99874: F, t99876: F, t99878: F, t99880: F, t99882: F, t99884: F, t99886: F, t99888: F, t99890: F) -> (F, F, F, F, F, F) {
    let t99892 = t28024 * t19908;
    let t99894 = t26938 * t29059;
    let t99896 = t18463 * t1196;
    let t99898 = t95391 * t19882;
    let t99900 = t20155 * t283;
    let t99901 = t99900 * t7749;
    let t99903 = t99865 / F::cast_from(27.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t99867 + t99869 / F::cast_from(12.0_f64) - t99871 / F::cast_from(64.0_f64) - t99874 / F::cast_from(32.0_f64) + t99876 / F::cast_from(8.0_f64) - t99878 / F::cast_from(128.0_f64) + t99880 / F::cast_from(288.0_f64) - t99882 / F::cast_from(48.0_f64) + t99884 / F::cast_from(96.0_f64) + t99886 / F::cast_from(6.0_f64) + t99888 / F::cast_from(6.0_f64) + t99890 / F::cast_from(144.0_f64) - t99892 / F::cast_from(288.0_f64) - t99894 / F::cast_from(72.0_f64) + t99896 / F::cast_from(24.0_f64) + F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t99898 - t99901 / F::cast_from(16.0_f64);
    (t99892, t99894, t99896, t99898, t99901, t99903)
}
