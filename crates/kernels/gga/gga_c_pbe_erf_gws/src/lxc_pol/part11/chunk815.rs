//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 815/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk815<F: Float>(t1: F, t4576: F, t550: F, t553: F, t6: F, t6045: F, t153: F, t413: F, t7236: F, t7271: F, t161: F, t148: F, t163: F, t5985: F, t547: F, t5984: F) -> (F, F, F, F, F, F, F, F) {
    let t18032 = t4576 * t1;
    let t18035 = 0.79015561315637923528e-2 * t550 * t18032 * t553;
    let t18046 = t6 * t6045;
    let t18049 = 0.17888888888888888889e-1 * t7271 + 0.22252592592592592592e0 * t7236 - 0.7316671043820612376e-1 * t413 + 0.15663796296296296297e-1 * t153 * t18046;
    let t18050 = t18049 * t161;
    let t18053 = 0.31505407223141117834e-1 * t148 * t18050 * t163;
    let t18067 = 0.756129773355386828e0 * t5985;
    let t18072 = 0.47461239486605618761e-3 * t5984 * t547;
    (t18032, t18035, t18046, t18049, t18050, t18053, t18067, t18072)
}
