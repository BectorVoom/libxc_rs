//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1195/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1195<F: Float>(t33: F, t265: F, t502: F, t127181: F, t132085: F, t127288: F, t1469: F, t33544: F, t35008: F, t4186: F, t57: F, t606: F, t8960: F, t118: F, t127340: F, t129308: F, t129436: F, t129437: F, t129438: F, t129440: F, t129445: F, t129447: F, t129449: F, t129452: F, t129455: F, t129457: F, t129459: F, t129461: F, t129463: F, t129465: F, t131384: F, t131387: F, t29459: F, t569: F, t7586: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t132086 = piecewise3::<f64>(t503, t132085, t127181);
    let t132093 = piecewise3::<f64>(t400, t127288, t132086 * t57 / F::new(2.0) - t33544 * t1469 / F::new(2.0) - t35008 * t606 / F::new(2.0) - t8960 * t4186 / F::new(2.0));
    let t132107 = -F::new(4.0) * t7586 * t29459 - F::new(2.0) * t129436 - F::new(2.0) * t129437 + F::new(6.0) * t129438 + (t131384 + t131387) * t569 - t118 * (t129308 + t132093) + F::new(6.0) * t129440 - F::new(4.0) * t129445 - F::new(4.0) * t129447 - F::new(4.0) * t129449 - F::new(4.0) * t129452 - F::new(2.0) * t129455 - t127340 - F::new(4.0) * t129457 - F::new(4.0) * t129459 - F::new(4.0) * t129461 - F::new(4.0) * t129463 - F::new(4.0) * t129465;
    t132107
}
