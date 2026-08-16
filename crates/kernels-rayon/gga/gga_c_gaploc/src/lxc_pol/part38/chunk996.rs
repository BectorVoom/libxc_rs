//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 996/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk996(t1445: f64, t1562: f64, t44474: f64, t13397: f64, t2487: f64, t6985: f64, t11318: f64, t2464: f64, t2465: f64, t587: f64, t2365: f64, t36211: f64, t7025: f64) -> (f64, f64, f64, f64) {
    let t46806 = 0.62115540045351614476e2_f64 * t1562 * t1445 * t44474;
    let t46811 = t2487 * t6985 * t13397;
    let t46815 = t587 * t2464 * t2465 * t11318;
    let t46818 = t7025 * t2365 * t36211;
    (t46806, t46811, t46815, t46818)
}
