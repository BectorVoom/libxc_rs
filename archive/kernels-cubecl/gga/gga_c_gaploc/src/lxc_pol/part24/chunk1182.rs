//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1182/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1182<F: Float>(t31754: F, t19532: F, t25723: F, t9074: F, t10163: F, t1358: F, t10172: F, t1079: F, t123: F, t1367: F, t30003: F, t30005: F, t30009: F, t30014: F, t30049: F, t31731: F, t31737: F, t31740: F, t31748: F, t3359: F, t3808: F, t488: F, t6507: F) -> F {
    let t31755 = F::cast_from(0.142275033178380748e-1_f64) * t31754;
    let t31757 = t9074 * t19532 * t25723;
    let t31758 = F::cast_from(0.71137516589190373998e-2_f64) * t31757;
    let t31759 = t1358 * t10163;
    let t31760 = F::cast_from(0.31616674039640166222e-2_f64) * t31759;
    let t31761 = t30003 - t30005 - t30009 - t30014 + F::cast_from(0.18970004423784099732e-1_f64) * t1358 * t31731 * t1367 - t31737 - F::cast_from(0.63233348079280332442e-2_f64) * t3808 * t10172 - F::cast_from(0.63233348079280332442e-2_f64) * t1358 * t31740 * t123 * t488 - F::cast_from(0.12646669615856066488e-1_f64) * t1079 * t3359 - F::cast_from(0.12646669615856066488e-1_f64) * t1358 * t6507 * t31748 - t30049 + t31755 - t31758 - t31760;
    t31761
}
