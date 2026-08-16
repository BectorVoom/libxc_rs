//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 846/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk846(t11413: f64, t2268: f64, t24139: f64, t6509: f64, t13265: f64, t2312: f64, t2854: f64, t31747: f64, t42726: f64, t42745: f64, t42748: f64, t42774: f64, t44513: f64, t44515: f64, t44516: f64, t44518: f64, t44521: f64, t44524: f64, t44527: f64, t44529: f64, t44530: f64, t44534: f64, t44538: f64, t6320: f64) -> f64 {
    let t44542 = 0.68292015925622759036e0_f64 * t2268 * t24139 * t11413 * t6509;
    let t44543 = t2312 * t13265;
    let t44544 = 0.35568758294595186999e-2_f64 * t44543;
    let t44545 = -0.3414600796281137952e0_f64 * t2268 * t6320 * t2854 * t31747 + 0.63233348079280332443e-2_f64 * t42726 + 0.47425011059460249332e-2_f64 * t42745 + 0.47425011059460249332e-2_f64 * t42748 - 0.63233348079280332443e-2_f64 * t42774 - t44513 + t44515 - t44516 - t44518 + t44521 + t44524 - t44527 + t44529 - t44530 + t44534 - t44538 + t44542 - t44544;
    t44545
}
