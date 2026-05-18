//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 793/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk793<F: Float>(t12526: F, t2487: F, t6985: F, t2365: F, t30209: F, t7025: F, t2610: F, t28023: F, t1843: F, t9647: F, t2563: F, t9756: F) -> (F, F, F, F, F) {
    let t40567 = t2487 * t6985 * t12526;
    let t40570 = t7025 * t2365 * t30209;
    let t40586 = t2610 * t28023;
    let t40588 = t9647 * t1843 * t40586;
    let t40591 = t9647 * t9756 * t2563;
    (t40567, t40570, t40586, t40588, t40591)
}
