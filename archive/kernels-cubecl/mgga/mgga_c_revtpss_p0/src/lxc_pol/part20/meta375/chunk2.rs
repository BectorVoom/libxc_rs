//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1360/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1360<F: Float>(t2721: F, t40324: F, t40326: F, t40333: F, t40337: F, t40340: F, t40345: F, t40349: F, t40355: F, t40357: F, t40361: F, t40365: F, t40367: F, t40369: F, t40374: F, t40376: F, t40381: F, t40385: F, t40390: F, t825: F, t827: F, t828: F) -> F {
    let t40392 = F::cast_from(0.51448821741683684368e-2_f64) * t40324 * t827 * t828 * t40326 - F::cast_from(0.50820002809285328224e-4_f64) * t40333 - F::cast_from(0.16262400898971305032e-2_f64) * t40337 + F::cast_from(0.12862205435420921092e-2_f64) * t2721 * t827 * t828 * t40340 - F::cast_from(0.24009450146119052704e-1_f64) * t40345 + F::cast_from(0.24009450146119052705e-1_f64) * t40349 - F::cast_from(0.30492001685571196935e-3_f64) * t40355 + F::cast_from(0.81312004494856525159e-3_f64) * t40357 + F::cast_from(0.15117061203111996148e0_f64) * t40361 - F::cast_from(0.50820002809285328224e-4_f64) * t40365 - F::cast_from(0.24009450146119052704e0_f64) * t40367 - F::cast_from(0.21437009059034868486e-3_f64) * t825 * t827 * t828 * t40369 + F::cast_from(0.81312004494856525159e-3_f64) * t40374 + F::cast_from(0.40015750243531754508e-2_f64) * t40376 + F::cast_from(0.68598428988911579156e-3_f64) * t40381 - F::cast_from(0.17149607247227894789e-3_f64) * t40385 - F::cast_from(0.34299214494455789577e-3_f64) * t40390;
    t40392
}
