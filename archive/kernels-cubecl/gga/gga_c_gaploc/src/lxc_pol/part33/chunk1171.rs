//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1171/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1171<F: Float>(t23763: F, t31735: F, t25722: F, t6508: F, t4261: F, t9074: F, t19532: F, t25723: F, t10163: F, t1358: F, t1367: F, t31543: F) -> (F, F, F, F, F, F) {
    let t31737 = F::cast_from(0.18970004423784099733e-1_f64) * t23763 * t31735;
    let t31752 = t6508 * t25722;
    let t31754 = t9074 * t4261 * t31752;
    let t31755 = F::cast_from(0.142275033178380748e-1_f64) * t31754;
    let t31757 = t9074 * t19532 * t25723;
    let t31758 = F::cast_from(0.71137516589190373998e-2_f64) * t31757;
    let t31759 = t1358 * t10163;
    let t31760 = F::cast_from(0.31616674039640166222e-2_f64) * t31759;
    let t31764 = t31543 * t1367;
    (t31737, t31752, t31755, t31758, t31760, t31764)
}
