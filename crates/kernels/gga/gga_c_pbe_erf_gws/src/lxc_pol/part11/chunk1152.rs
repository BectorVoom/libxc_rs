//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1152/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1152<F: Float>(t3553: F, t1792: F, t186: F, t211: F, t16985: F, t40962: F, t47942: F, t47946: F, t47951: F, t47955: F, t47959: F, t47994: F, t47997: F, t48000: F, t48003: F) -> (F, F) {
    let t48326 = t3553 * t3553;
    let t48330 = F::new(4.0) / F::new(5.0) * t211 * t186 * t1792 * t48326;
    let t48341 = -F::new(0.5037777777777777778e-2) * t40962 + F::new(0.45340000000000000001e-1) * t47942 - F::new(0.45340000000000000002e-1) * t47994 + F::new(0.37783333333333333335e-2) * t47946 + F::new(0.5037777777777777778e-2) * t47997 - F::new(0.4534e-1) * t47951 + F::new(0.6801e-1) * t48000 - F::new(0.11335e-1) * t47955 - F::new(0.15113333333333333333e-1) * t48003 - t16985 + F::new(0.55975308641975308645e-2) * t47959;
    (t48330, t48341)
}
