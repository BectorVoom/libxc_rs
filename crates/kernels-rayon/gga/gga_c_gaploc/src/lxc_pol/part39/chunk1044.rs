//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1044/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1044(t13033: f64, t4614: f64, t5748: f64, t1445: f64, t2087: f64, t33118: f64, t935: f64, t1991: f64, t42944: f64, t590: f64, t739: f64, t1890: f64, t1966: f64) -> (f64, f64, f64, f64) {
    let t43864 = 0.36809208915763919689e2_f64 * t5748 * t4614 * t13033;
    let t43870 = t2087 * t1445 * t33118 * t935;
    let t43875 = 0.20449560508757733161e1_f64 * t1991 * t739 * t42944 * t590;
    let t43879 = 0.97135412416599232513e1_f64 * t1966 * t1890 * t42944 * t590;
    (t43864, t43870, t43875, t43879)
}
