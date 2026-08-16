//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1162/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1162<F: Float>(t91828: F, t91830: F, t91832: F, t91835: F, t91837: F, t91839: F, t91841: F, t91844: F, t91847: F, t91850: F, t91852: F, t91854: F, t91857: F, t91859: F) -> F {
    let t92149 = -F::cast_from(0.485625e0_f64) * t91828 + F::cast_from(0.1875e0_f64) * t91830 + F::cast_from(0.225e1_f64) * t91832 + F::cast_from(0.97125e1_f64) * t91835 + F::cast_from(0.2428125e0_f64) * t91837 - F::cast_from(0.1875e0_f64) * t91839 - F::cast_from(0.809375e-1_f64) * t91841 - F::cast_from(0.97125e0_f64) * t91844 + F::cast_from(0.485625e1_f64) * t91847 + F::cast_from(0.485625e0_f64) * t91850 - F::cast_from(0.45e1_f64) * t91852 - F::cast_from(0.19425e1_f64) * t91854 + F::cast_from(0.19425e1_f64) * t91857 + F::cast_from(0.3375e1_f64) * t91859;
    t92149
}
