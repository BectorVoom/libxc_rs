//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 846/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk846<F: Float>(t11413: F, t2268: F, t24139: F, t6509: F, t13265: F, t2312: F, t2854: F, t31747: F, t42726: F, t42745: F, t42748: F, t42774: F, t44513: F, t44515: F, t44516: F, t44518: F, t44521: F, t44524: F, t44527: F, t44529: F, t44530: F, t44534: F, t44538: F, t6320: F) -> F {
    let t44542 = F::new(0.68292015925622759036e0) * t2268 * t24139 * t11413 * t6509;
    let t44543 = t2312 * t13265;
    let t44544 = F::new(0.35568758294595186999e-2) * t44543;
    let t44545 = -F::new(0.3414600796281137952e0) * t2268 * t6320 * t2854 * t31747 + F::new(0.63233348079280332443e-2) * t42726 + F::new(0.47425011059460249332e-2) * t42745 + F::new(0.47425011059460249332e-2) * t42748 - F::new(0.63233348079280332443e-2) * t42774 - t44513 + t44515 - t44516 - t44518 + t44521 + t44524 - t44527 + t44529 - t44530 + t44534 - t44538 + t44542 - t44544;
    t44545
}
