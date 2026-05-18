//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1133/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1133<F: Float>(t1358: F, t9205: F, t20513: F, t4261: F, t9074: F, t20521: F, t1365: F, t20358: F, t6525: F, t19532: F, t20370: F, t2300: F, t23983: F, t6455: F) -> (F, F, F, F, F, F) {
    let t30120 = F::new(0.18970004423784099732e-1) * t1358 * t9205;
    let t30123 = F::new(0.94850022118920498664e-2) * t9074 * t4261 * t20513;
    let t30126 = F::new(0.47425011059460249332e-2) * t9074 * t4261 * t20521;
    let t30129 = F::new(0.23712505529730124666e-2) * t6525 * t1365 * t20358;
    let t30132 = F::new(0.142275033178380748e-1) * t9074 * t19532 * t20370;
    let t30135 = F::new(0.47425011059460249332e-2) * t23983 * t2300 * t6455;
    (t30120, t30123, t30126, t30129, t30132, t30135)
}
