//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1626/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1626<F: Float>(t40325: F, t87399: F, t10871: F, t10698: F, t10870: F, t1544: F, t23148: F, t2477: F, t2721: F, t40324: F, t51170: F, t5962: F, t5966: F, t62251: F, t62399: F, t62401: F, t62431: F, t62443: F, t62445: F, t76878: F, t76882: F, t76887: F, t77127: F, t77131: F, t825: F, t827: F, t828: F, t851: F, t87395: F, t87400: F) -> (F, F, F) {
    let t87764 = t87399 * t40325;
    let t87775 = t87399 * t10871;
    let t87783 = F::cast_from(0.17149607247227894789e-1_f64) * t851 * t2477 * t828 * t23148 * t1544 - F::cast_from(0.21437009059034868486e-3_f64) * t825 * t827 * t828 * t87400 - F::cast_from(0.1084295579938911763e-3_f64) * t62251 - F::cast_from(0.34013387707001991332e-1_f64) * t62399 + F::cast_from(0.68026775414003982664e-1_f64) * t62401 - F::cast_from(0.1543464652250510531e0_f64) * t851 * t10698 * t828 * t5966 * t5962 + F::cast_from(0.12862205435420921092e-2_f64) * t2721 * t827 * t828 * t87395 + F::cast_from(0.51448821741683684368e-2_f64) * t40324 * t827 * t828 * t87764 + F::cast_from(0.11560105625909173524e-1_f64) * t51170 + F::cast_from(0.11433071498151929859e-3_f64) * t76878 + F::cast_from(0.17149607247227894789e-2_f64) * t76882 - F::cast_from(0.50820002809285328224e-4_f64) * t76887 - F::cast_from(0.50820002809285328224e-4_f64) * t77127 - F::cast_from(0.30492001685571196935e-3_f64) * t77131 - F::cast_from(0.77173232612525526552e-2_f64) * t10870 * t827 * t828 * t87775 - F::cast_from(0.16262400898971305032e-1_f64) * t62431 + F::cast_from(0.91464571985215438873e-3_f64) * t62443 - F::cast_from(0.45732285992607719437e-3_f64) * t62445;
    (t87764, t87775, t87783)
}
