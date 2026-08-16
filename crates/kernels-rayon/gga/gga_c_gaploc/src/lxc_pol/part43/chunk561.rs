//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 561/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk561(t3281: f64, t5676: f64, t2530: f64, t2610: f64, t2365: f64, t2033: f64, t1445: f64, t9596: f64, t1457: f64, t3209: f64, t325: f64, t701: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9942 = 0.29792074959875355558e-1_f64 * t5676 * t3281;
    let t9943 = t2610 * t2530;
    let t9944 = t2365 * t9943;
    let t9946 = 0.29792074959875355558e-1_f64 * t2033 * t9944;
    let t9947 = t1445 * t9596;
    let t9950 = t1457 * t9596;
    let t9953 = t325 * t3209;
    let t9954 = t9953 * t701;
    (t9942, t9946, t9947, t9950, t9953, t9954)
}
