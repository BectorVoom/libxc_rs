//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1183/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1183<F: Float>(t1701: F, t290: F, t6: F, t2035: F, t39: F, t5230: F, t1110: F, t51: F, t5284: F, t1111: F, t1209: F, t14742: F, t22003: F, t22065: F, t22081: F, t22082: F, t281: F, t287: F, t291: F, t5003: F, t5009: F, t5265: F, t5267: F, t70463: F, t70653: F, t70779: F, t83356: F, t88439: F, t88442: F, t88909: F, t90049: F, t90159: F) -> F {
    let t90280 = t290 * t6 * t1701;
    let t90288 = t5230 * t39 * t2035;
    let t90293 = t5284 * t6 * t51 * t1110;
    let t90300 = F::new(0.45910941751869106328e2) * t22082 * t5003 + F::new(0.22341601828860387373e3) * t5265 * t5009 * t88909 * t291 + F::new(0.14498192132169191472e2) * t22081 * t1209 * t1111 - F::new(0.14498192132169191472e2) * t22065 * t1111 + F::new(0.19686723316703981795e0) * t281 * t88439 * t88442 * t287 * t90280 - F::new(0.14498192132169191472e2) * t14742 * t90159 + F::new(0.70065858367097548785e2) * t70779 * t90049 + F::new(0.87582322958871935983e1) * t90288 * t5267 - F::new(0.28996384264338382944e2) * t70653 * t90293 + F::new(0.28996384264338382944e2) * t70463 * t90293 + F::new(0.14498192132169191472e2) * t83356 * t22003;
    t90300
}
