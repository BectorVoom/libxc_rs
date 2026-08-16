//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1235/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1235<F: Float>(t20994: F, t2563: F, t20944: F, t41011: F, t13278: F, t5614: F, t20963: F, t9667: F, t46881: F, t5587: F, t20908: F, t2697: F) -> (F, F, F, F, F, F) {
    let t67920 = t2563 * t20994;
    let t67937 = t41011 * t20944;
    let t67976 = t13278 * t5614;
    let t67978 = t9667 * t20963;
    let t67980 = t46881 * t5587;
    let t68021 = t2697 * t20908;
    (t67920, t67937, t67976, t67978, t67980, t68021)
}
