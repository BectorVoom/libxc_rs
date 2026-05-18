//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1141/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1141<F: Float>(t14565: F, t14567: F, t1135: F, t3065: F, t2134: F, t14059: F, t14073: F, t14080: F, t14085: F, t14554: F, t14556: F, t14558: F, t14560: F, t14563: F) -> (F, F) {
    let t14568 = t14565 * t14567;
    let t14570 = t3065 * t1135;
    let t14571 = t2134 * t14570;
    let t14574 = F::new(7.0) / F::new(288.0) * t14554 - t14556 / F::new(384.0) + F::new(7.0) / F::new(576.0) * t14558 - t14560 / F::new(192.0) + F::new(7.0) / F::new(576.0) * t14059 + F::new(7.0) / F::new(144.0) * t14563 + t14568 / F::new(96.0) - t14571 / F::new(96.0) + t14073 + F::new(7.0) / F::new(1152.0) * t14080 + t14085;
    (t14570, t14574)
}
