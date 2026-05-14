//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 737/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk737<F: Float>(t13310: F, t2312: F, t42825: F, t1063: F, t11259: F, t6320: F, t6519: F, t2268: F, t2854: F, t31585: F, t11413: F, t24139: F, t6509: F, t13265: F, t31747: F, t42726: F, t42745: F, t42748: F, t42774: F, t44513: F, t44515: F, t44516: F, t44518: F, t44521: F, t44524: F, t44527: F) -> (F,) {
    let t44528 = t2312 * t13310;
    let t44529 = 0.11856252764865062333e-2 * t44528;
    let t44530 = 0.12646669615856066489e-1 * t42825;
    let t44534 = 0.17073003981405689759e0 * t1063 * t6320 * t11259 * t6519;
    let t44538 = 0.34146007962811379518e0 * t2268 * t6320 * t2854 * t31585;
    let t44542 = 0.68292015925622759036e0 * t2268 * t24139 * t11413 * t6509;
    let t44543 = t2312 * t13265;
    let t44544 = 0.35568758294595186999e-2 * t44543;
    let t44545 = -0.3414600796281137952e0 * t2268 * t6320 * t2854 * t31747 + 0.63233348079280332443e-2 * t42726 + 0.47425011059460249332e-2 * t42745 + 0.47425011059460249332e-2 * t42748 - 0.63233348079280332443e-2 * t42774 - t44513 + t44515 - t44516 - t44518 + t44521 + t44524 - t44527 + t44529 - t44530 + t44534 - t44538 + t44542 - t44544;
    (t44545,)
}
