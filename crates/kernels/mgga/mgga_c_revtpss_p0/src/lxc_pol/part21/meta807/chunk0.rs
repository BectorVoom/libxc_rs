//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2943/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2943<F: Float>(t1011: F, t4886: F, t697: F, t1065: F, t372: F, t4866: F, t11774: F, t16103: F, t42254: F, t42257: F, t42259: F, t42268: F, t42270: F, t42274: F, t42282: F, t42284: F, t42288: F) -> (F, F) {
    let t53542 = t1011 * t697 * t4886;
    let t53543 = t53542 / F::new(432.0);
    let t53545 = t372 * t1065 * t4866;
    let t53549 = -t42254 / F::new(432.0) - t42257 / F::new(324.0) + F::cast_from(0.85748036236139473944e-3_f64) * t42259 - F::cast_from(0.42874018118069736972e-3_f64) * t42268 - F::cast_from(0.15244095330869239812e-2_f64) * t42270 - F::cast_from(0.19055119163586549765e-3_f64) * t42274 + F::cast_from(0.45732285992607719436e-2_f64) * t42282 - F::cast_from(0.42874018118069736972e-3_f64) * t42284 - F::cast_from(0.14291339372689912324e-3_f64) * t42288 - t53543 - F::cast_from(0.85748036236139473944e-3_f64) * t11774 * t53545 * t16103;
    (t53545, t53549)
}
