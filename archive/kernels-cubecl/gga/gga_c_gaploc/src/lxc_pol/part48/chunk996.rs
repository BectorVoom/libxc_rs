//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 996/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk996<F: Float>(t1445: F, t1562: F, t44474: F, t13397: F, t2487: F, t6985: F, t11318: F, t2464: F, t2465: F, t587: F, t2365: F, t36211: F, t7025: F) -> (F, F, F, F) {
    let t46806 = F::cast_from(0.62115540045351614476e2_f64) * t1562 * t1445 * t44474;
    let t46811 = t2487 * t6985 * t13397;
    let t46815 = t587 * t2464 * t2465 * t11318;
    let t46818 = t7025 * t2365 * t36211;
    (t46806, t46811, t46815, t46818)
}
