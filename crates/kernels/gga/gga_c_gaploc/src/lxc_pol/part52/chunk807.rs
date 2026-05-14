//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 807/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk807<F: Float>(t42431: F, t1445: F, t1562: F, t44474: F, t13397: F, t2487: F, t6985: F, t11318: F, t2464: F, t2465: F, t587: F, t2365: F, t36211: F, t7025: F, t10430: F, t9263: F, t993: F) -> (F, F, F, F, F, F) {
    let t46799 = 0.19171462976960374838e0 * t42431;
    let t46806 = 0.62115540045351614476e2 * t1562 * t1445 * t44474;
    let t46811 = t2487 * t6985 * t13397;
    let t46815 = t587 * t2464 * t2465 * t11318;
    let t46818 = t7025 * t2365 * t36211;
    let t46819 = 0.14896037479937677779e-1 * t46818;
    let t46821 = t9263 * t993 * t10430;
    (t46799, t46806, t46811, t46815, t46819, t46821)
}
