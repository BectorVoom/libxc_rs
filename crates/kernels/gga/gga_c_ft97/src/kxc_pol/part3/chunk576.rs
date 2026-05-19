//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 576/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk576<F: Float>(t140: F, t4699: F, t527: F, t1013: F, t2058: F, t133: F, t2066: F, t3086: F, t4481: F, t4485: F, t4489: F, t550: F, t2001: F, t4675: F, t4677: F) -> (F, F, F, F, F, F, F, F) {
    let t141 = F::new(0.1e-59) < t140;
    let t4700 = t527 * t4699;
    let t4702 = t1013 * t1013;
    let t4703 = t2058 * t4702;
    let t4704 = t133 * t4703;
    let t4710 = -t2066 + F::cast_from(0.11113000182098765433e-1_f64) * t3086 + F::cast_from(0.22226000364197530865e-1_f64) * t4481 - F::cast_from(0.33339000546296296298e-1_f64) * t4485 + F::cast_from(0.16669500273148148149e-1_f64) * t4489;
    let t4711 = t550 * t4710;
    let t4712 = t133 * t4711;
    let t4714 = piecewise3::<F>(t141, -F::new(4.0) * t2001 * t4677 + F::new(2.0) * t4675 + F::new(2.0) * t4700 + F::new(2.0) * t4704 - t4712, F::new(0.0));
    (t4700, t4702, t4703, t4704, t4710, t4711, t4712, t4714)
}
