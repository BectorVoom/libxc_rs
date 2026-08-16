//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1360/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1360(t2721: f64, t40324: f64, t40326: f64, t40333: f64, t40337: f64, t40340: f64, t40345: f64, t40349: f64, t40355: f64, t40357: f64, t40361: f64, t40365: f64, t40367: f64, t40369: f64, t40374: f64, t40376: f64, t40381: f64, t40385: f64, t40390: f64, t825: f64, t827: f64, t828: f64) -> f64 {
    let t40392 = 0.51448821741683684368e-2_f64 * t40324 * t827 * t828 * t40326 - 0.50820002809285328224e-4_f64 * t40333 - 0.16262400898971305032e-2_f64 * t40337 + 0.12862205435420921092e-2_f64 * t2721 * t827 * t828 * t40340 - 0.24009450146119052704e-1_f64 * t40345 + 0.24009450146119052705e-1_f64 * t40349 - 0.30492001685571196935e-3_f64 * t40355 + 0.81312004494856525159e-3_f64 * t40357 + 0.15117061203111996148e0_f64 * t40361 - 0.50820002809285328224e-4_f64 * t40365 - 0.24009450146119052704e0_f64 * t40367 - 0.21437009059034868486e-3_f64 * t825 * t827 * t828 * t40369 + 0.81312004494856525159e-3_f64 * t40374 + 0.40015750243531754508e-2_f64 * t40376 + 0.68598428988911579156e-3_f64 * t40381 - 0.17149607247227894789e-3_f64 * t40385 - 0.34299214494455789577e-3_f64 * t40390;
    t40392
}
