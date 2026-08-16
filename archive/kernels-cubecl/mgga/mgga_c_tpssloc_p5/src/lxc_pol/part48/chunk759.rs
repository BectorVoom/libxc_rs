//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 759/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk759<F: Float>(t7025: F, t9231: F, t6486: F, t7032: F, t240: F, t67: F, t1864: F, t1860: F, t6509: F, t7031: F, t2031: F, t22489: F) -> (F, F, F, F, F) {
    let t23975 = t9231 * t7025;
    let t23978 = t6486 * t7032;
    let t23992 = t240 * t67;
    let t23993 = t23992 * t1864;
    let t23995 = F::cast_from(88.0_f64) / F::cast_from(27.0_f64) * t1860 * t23993;
    let t23998 = t7031 * t6509;
    let t23999 = t1860 * t23998;
    let t24001 = t2031 * t22489;
    (t23975, t23978, t23995, t23999, t24001)
}
