//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 696/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk696<F: Float>(t13261: F, t1445: F, t597: F, t2366: F, t3529: F, t2365: F, t1429: F, t11426: F, t6590: F, t3516: F, t6508: F, t4391: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13383 = t1445 * t13261;
    let t13385 = F::cast_from(0.11502877786176224903e2_f64) * t597 * t13383;
    let t13386 = t2366 * t3529;
    let t13387 = t2365 * t13386;
    let t13388 = t1429 * t13387;
    let t13389 = F::cast_from(0.14896037479937677779e-1_f64) * t13388;
    let t13390 = t11426 * t6590;
    let t13392 = t6508 * t3516;
    let t13393 = t2365 * t13392;
    let t13394 = t4391 * t13393;
    (t13383, t13385, t13386, t13387, t13389, t13390, t13392, t13393, t13394)
}
