//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 981/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk981<F: Float>(t2210: F, t30412: F, t23571: F, t4733: F, t12968: F, t13153: F, t6626: F, t27006: F, t925: F, t2221: F, t4462: F, t5855: F, t4454: F, t9115: F, t574: F, t5935: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t30413 = t2210 * t30412;
    let t30416 = t23571 * t4733;
    let t30417 = t12968 * t30416;
    let t30420 = t13153 * t6626;
    let t30423 = t27006 * t925;
    let t30424 = t2221 * t30423;
    let t30427 = t5855 * t4462;
    let t30428 = t2221 * t30427;
    let t30431 = t5855 * t4454;
    let t30432 = t9115 * t30431;
    let t30436 = t574 * t5935 * t4733;
    (t30413, t30416, t30417, t30420, t30423, t30424, t30427, t30428, t30431, t30432, t30436)
}
