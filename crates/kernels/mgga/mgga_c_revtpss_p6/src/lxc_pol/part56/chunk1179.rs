//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1179/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1179<F: Float>(t105364: F, t33494: F, t1042: F, t105236: F, t105460: F, t1203: F, t124590: F, t124611: F, t124612: F, t124632: F, t124645: F, t124684: F, t124711: F, t124755: F, t124802: F, t124862: F, t124893: F, t124898: F, t1769: F, t1789: F, t31993: F, t32015: F, t33433: F, t33498: F, t34899: F, t34952: F, t34991: F, t3719: F, t5236: F, t5401: F, t5405: F, t5406: F) -> F {
    let t131518 = t105364 * t33494;
    let t131552 = F::cast_from(0.24791552806034007214e-3_f64) * t124632 - F::cast_from(0.56468933516960933998e-3_f64) * t124711 * t32015 * t124612 * t105236 - F::cast_from(0.5578099381357651623e-3_f64) * t131518 * t33498 + F::cast_from(0.56468933516960933998e-3_f64) * t124755 * t32015 * t124612 * t5236 + F::cast_from(0.15058382271189582399e-2_f64) * t34991 * t33433 - F::cast_from(0.11156198762715303246e-2_f64) * t124684 * t31993 * t3719 * t1769 * t1203 - F::cast_from(0.112937867033921868e-2_f64) * t124862 * t32015 * t124645 * t105460 + F::cast_from(0.12395776403017003607e-3_f64) * t124590 * t34952 + F::cast_from(0.11156198762715303246e-2_f64) * t124802 * t1042 * t1789 * t5405 - F::cast_from(0.18822977838986977999e-3_f64) * t124611 * t124898 * t5401 + F::cast_from(0.7437465841810202164e-3_f64) * t124893 * t1042 * t34899 * t5405 - F::cast_from(0.18822977838986977999e-3_f64) * t124611 * t124898 * t5406;
    t131552
}
