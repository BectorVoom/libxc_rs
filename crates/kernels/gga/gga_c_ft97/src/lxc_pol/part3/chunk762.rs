//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 762/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk762<F: Float>(t15874: F, t420: F, t419: F, t15847: F, t15850: F, t15852: F, t15855: F, t15858: F, t15861: F, t15863: F, t15866: F, t15869: F, t15872: F) -> (F, F) {
    let t15875 = t420 * t15874;
    let t15876 = t419 * t15875;
    let t15878 = -F::cast_from(0.51074886703703703704e-1_f64) * t15847 + F::cast_from(0.34049924469135802469e-1_f64) * t15850 + F::cast_from(0.34049924469135802469e-1_f64) * t15852 - F::cast_from(0.42562405586419753087e-2_f64) * t15855 + F::cast_from(0.38306165027777777778e-1_f64) * t15858 - F::cast_from(0.51074886703703703704e-1_f64) * t15861 - F::cast_from(0.17024962234567901235e-1_f64) * t15863 + F::cast_from(0.21281202793209876543e-2_f64) * t15866 + F::cast_from(0.85124811172839506173e-2_f64) * t15869 - F::cast_from(0.12768721675925925926e-1_f64) * t15872 + F::cast_from(0.6384360837962962963e-2_f64) * t15876;
    (t15876, t15878)
}
