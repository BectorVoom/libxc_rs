//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 762/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk762(t12953: f64, t9272: f64, t3177: f64, t993: f64, t9263: f64, t2890: f64, t9267: f64, t123: f64, t3338: f64, t883: f64, t912: f64, t587: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12954 = t9272 * t12953;
    let t12955 = 0.11502877786176224903e1_f64 * t12954;
    let t12957 = t993 * t3177;
    let t12958 = t9263 * t12957;
    let t12959 = 0.76685851907841499353e0_f64 * t12958;
    let t12960 = t2890 * t3177;
    let t12961 = t9267 * t12960;
    let t12962 = 0.19171462976960374838e1_f64 * t12961;
    let t12963 = t3338 * t123;
    let t12964 = t12963 * t883;
    let t12965 = t912 * t12964;
    let t12966 = t587 * t12965;
    (t12955, t12957, t12959, t12960, t12962, t12963, t12964, t12965, t12966)
}
