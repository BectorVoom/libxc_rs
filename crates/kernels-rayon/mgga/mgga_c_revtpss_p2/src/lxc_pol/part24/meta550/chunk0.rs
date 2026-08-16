//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1626/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1626(t40325: f64, t87399: f64, t10871: f64, t10698: f64, t10870: f64, t1544: f64, t23148: f64, t2477: f64, t2721: f64, t40324: f64, t51170: f64, t5962: f64, t5966: f64, t62251: f64, t62399: f64, t62401: f64, t62431: f64, t62443: f64, t62445: f64, t76878: f64, t76882: f64, t76887: f64, t77127: f64, t77131: f64, t825: f64, t827: f64, t828: f64, t851: f64, t87395: f64, t87400: f64) -> (f64, f64, f64) {
    let t87764 = t87399 * t40325;
    let t87775 = t87399 * t10871;
    let t87783 = 0.17149607247227894789e-1_f64 * t851 * t2477 * t828 * t23148 * t1544 - 0.21437009059034868486e-3_f64 * t825 * t827 * t828 * t87400 - 0.1084295579938911763e-3_f64 * t62251 - 0.34013387707001991332e-1_f64 * t62399 + 0.68026775414003982664e-1_f64 * t62401 - 0.1543464652250510531e0_f64 * t851 * t10698 * t828 * t5966 * t5962 + 0.12862205435420921092e-2_f64 * t2721 * t827 * t828 * t87395 + 0.51448821741683684368e-2_f64 * t40324 * t827 * t828 * t87764 + 0.11560105625909173524e-1_f64 * t51170 + 0.11433071498151929859e-3_f64 * t76878 + 0.17149607247227894789e-2_f64 * t76882 - 0.50820002809285328224e-4_f64 * t76887 - 0.50820002809285328224e-4_f64 * t77127 - 0.30492001685571196935e-3_f64 * t77131 - 0.77173232612525526552e-2_f64 * t10870 * t827 * t828 * t87775 - 0.16262400898971305032e-1_f64 * t62431 + 0.91464571985215438873e-3_f64 * t62443 - 0.45732285992607719437e-3_f64 * t62445;
    (t87764, t87775, t87783)
}
