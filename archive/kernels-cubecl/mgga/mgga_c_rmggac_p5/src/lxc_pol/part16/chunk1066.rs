//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1066/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1066<F: Float>(t38552: F, t38554: F, t38556: F, t42665: F, t44821: F, t44825: F, t44828: F, t44831: F, t44835: F, t44838: F, t44841: F, t44844: F, t44847: F, t44850: F, t44854: F, t44857: F, t44860: F) -> F {
    let t48212 = -F::cast_from(0.1440846329149835838e-2_f64) * t44821 + F::cast_from(0.12195059916630011325e-2_f64) * t38552 - F::cast_from(0.1440846329149835838e-2_f64) * t44825 - F::cast_from(0.1440846329149835838e-2_f64) * t44828 + F::cast_from(0.12195059916630011325e-2_f64) * t38554 + F::cast_from(0.3842256877732895568e-2_f64) * t44831 - F::cast_from(0.72042316457491791901e-3_f64) * t44835 - F::cast_from(0.72042316457491791901e-3_f64) * t44838 - F::cast_from(0.72042316457491791901e-3_f64) * t44841 - F::cast_from(0.1440846329149835838e-2_f64) * t44844 - F::cast_from(0.1440846329149835838e-2_f64) * t44847 - F::cast_from(0.1440846329149835838e-2_f64) * t44850 - F::cast_from(0.14088275218353950416e-1_f64) * t38556 + F::cast_from(0.3842256877732895568e-2_f64) * t44854 - F::cast_from(0.30487649791575028312e-3_f64) * t44857 - F::cast_from(0.72042316457491791901e-3_f64) * t44860 - t42665;
    t48212
}
