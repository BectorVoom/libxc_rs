//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1112/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1112(t28386: f64, t28446: f64, t28498: f64, t28554: f64, t589: f64, t1505: f64, t8182: f64, t1555: f64, t2069: f64, t27491: f64, t27494: f64, t5900: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28556 = t28386 + t28446 + t28498 + t28554;
    let t28557 = t28556 * t589;
    let t28558 = t8182 * t1505;
    let t28559 = t28558 * t1555;
    let t28560 = t27491 * t2069;
    let t28562 = 2.0_f64 * t27494 * t5900;
    (t28556, t28557, t28558, t28559, t28560, t28562)
}
