//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 970/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk970<F: Float>(t2581: F, t7130: F, t2567: F, t2615: F, t2579: F, t34: F, t7694: F, t1820: F, t1648: F, t3415: F, t10907: F, t10912: F, t10915: F, t10919: F, t10921: F, t10923: F, t10926: F, t10929: F, t10932: F, t10934: F, t10937: F, t10942: F, t7784: F) -> (F, F, F, F, F) {
    let t10944 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t7130 * t2581;
    let t10946 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t2615 * t2567;
    let t10947 = t2579 * t34;
    let t10948 = t7694 * t10947;
    let t10950 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t1820 * t10948;
    let t10952 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1648 * t3415;
    let t10953 = t10907 - t10912 - t10915 - t10919 - t10921 - t10923 + t10926 + t10929 - t10932 + t10934 - t7784 - t10937 - t10942 + t10944 - t10946 + t10950 - t10952;
    (t10944, t10946, t10950, t10952, t10953)
}
