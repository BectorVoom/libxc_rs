//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1175/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1175<F: Float>(t14752: F, t14506: F, t14520: F, t14551: F, t14554: F, t14558: F, t14563: F, t3703: F, t3944: F, t1105: F, t14390: F, t1167: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14999 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t14752;
    let t15050 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t14506;
    let t15057 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t14520;
    let t15070 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t14551;
    let t15072 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t14554;
    let t15074 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t14558;
    let t15076 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t14563;
    let t15118 = t3944 * t3703;
    let t15121 = t14390 * t1105;
    let t15124 = t1105 * t1167;
    (t14999, t15050, t15057, t15070, t15072, t15074, t15076, t15118, t15121, t15124)
}
