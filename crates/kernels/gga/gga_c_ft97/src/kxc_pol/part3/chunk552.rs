//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 552/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk552<F: Float>(t204: F, t41: F, t4995: F, t5003: F, t237: F, t1100: F, t213: F, t11: F, t215: F, t207: F, t1690: F, t1127: F, t3780: F, t2427: F, t14: F, t228: F, t231: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5005 = 0.44057546758024691357e0 * t41 * t204 * t4995 + 0.18770038718167957794e-1 * t5003;
    let t5006 = t237 * t5005;
    let t5007 = t1100 * t5006;
    let t5009 = t213 * t213;
    let t5010 = t11 * t5009;
    let t5011 = t215 * t215;
    let t5014 = 1.0 / t207 / t5011 / t215;
    let t5016 = t1690 * t5010 * t5014;
    let t5019 = t3780 * t1127;
    let t5025 = t1127 * t1127;
    let t5026 = t2427 * t5025;
    let t5029 = t4995 * t14;
    let t5031 = t228 * t5029 * t231;
    (t5005, t5007, t5009, t5014, t5016, t5019, t5025, t5026, t5031)
}
