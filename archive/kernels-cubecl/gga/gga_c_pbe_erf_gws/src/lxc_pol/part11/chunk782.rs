//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 782/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk782<F: Float>(t12682: F, t12700: F, t598: F, t186: F, t185: F, t2741: F, t3564: F, t12344: F, t220: F, t616: F, t10743: F, t10938: F, t950: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12701 = t12682 + t12700;
    let t12702 = t598 * t12701;
    let t12703 = t186 * t12702;
    let t12705 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t185 * t12703;
    let t12707 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t2741 * t3564;
    let t12709 = -F::cast_from(3.0_f64) * t12344;
    let t12710 = t220 * t12709;
    let t12711 = t186 * t12710;
    let t12713 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t616 * t12711;
    let t12715 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t10743 * t3564;
    let t12716 = t10938 * t950;
    (t12701, t12702, t12703, t12705, t12707, t12709, t12710, t12711, t12713, t12715, t12716)
}
