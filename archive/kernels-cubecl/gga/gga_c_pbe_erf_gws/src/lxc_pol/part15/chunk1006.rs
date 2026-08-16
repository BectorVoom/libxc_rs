//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1006/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1006<F: Float>(t1114: F, t6644: F, t6648: F, t3134: F, t6217: F, t3148: F, t6484: F, t6485: F, t4341: F, t4349: F, t4503: F, t4506: F, t4513: F, t4539: F, t4542: F, t6907: F, t6911: F, t6918: F, t6923: F, t6929: F, t6932: F, t6966: F, t6969: F, t7984: F, t8517: F, t8521: F) -> (F, F, F, F, F, F) {
    let t9035 = t1114 * t6644;
    let t9037 = t9035 * t6648 / F::cast_from(48.0_f64);
    let t9039 = t6217 * t3134 / F::cast_from(96.0_f64);
    let t9041 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t6484 * t3148;
    let t9042 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t6485;
    let t9043 = t6907 + t4341 - t6911 - t4349 + t6918 + t4503 - t4506 - t4513 + t4539 - t6923 + t4542 - t6929 + t6932 + t6966 + t6969 - t8517 - t7984 - t8521;
    (t9035, t9037, t9039, t9041, t9042, t9043)
}
