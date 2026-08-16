//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2804/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2804<F: Float>(t10541: F, t14495: F, t2782: F, t10518: F, t14568: F, t1568: F, t4503: F, t786: F, t10532: F, t51519: F, t51522: F, t51523: F, t51527: F, t51531: F, t51535: F, t51538: F, t51541: F) -> F {
    let t51544 = t2782 * t10541 * t14495;
    let t51546 = t14568 * t10518;
    let t51547 = F::cast_from(0.39029762157531132076e-1_f64) * t51546;
    let t51548 = t4503 * t1568;
    let t51549 = t786 * t51548;
    let t51550 = t51549 * t10532;
    let t51552 = F::cast_from(0.16463622957338778996e-1_f64) * t51519 + t51522 - F::cast_from(0.29272321618148349057e-1_f64) * t51523 + F::cast_from(0.16463622957338778996e-1_f64) * t51527 - F::cast_from(0.65854491829355115984e-1_f64) * t51531 + F::cast_from(0.11708928647259339623e0_f64) * t51535 + t51538 + F::cast_from(0.58544643236296698112e-1_f64) * t51541 + F::cast_from(0.32927245914677557992e-1_f64) * t51544 + t51547 + F::cast_from(0.58544643236296698113e-1_f64) * t51550;
    t51552
}
