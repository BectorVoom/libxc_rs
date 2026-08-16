//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 616/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk616<F: Float>(t2702: F, t626: F, t1045: F, t1055: F, t184: F, t188: F, t2671: F, t2679: F, t622: F, t634: F, t135: F, t144: F, t1501: F, t1510: F, t1520: F, t1530: F, t1534: F, t1544: F, t1547: F, t1553: F, t2535: F, t2536: F, t2537: F, t2559: F, t2575: F, t2606: F, t2608: F, t2611: F, t560: F, t637: F, t639: F) -> (F, F, F) {
    let t2703 = t626 * t2702;
    let t2706 = F::cast_from(0.65854491829355115987e0_f64) * t2671 * t188 - F::cast_from(0.65854491829355115987e0_f64) * t1045 * t634 - F::cast_from(0.65854491829355115987e0_f64) * t622 * t1055 + F::cast_from(0.13170898365871023197e1_f64) * t184 * t2679 - F::cast_from(0.65854491829355115987e0_f64) * t184 * t2703;
    let t2710 = t135 * t144 * t2706 * t639 + F::cast_from(3.0_f64) * t135 * t2575 * t560 - t2536 * t2537 * t637 - t1501 - t1510 - t1520 + t1530 + t1534 + t1544 - t1547 - t1553 - t2535 + t2559 + t2606 + t2608 - t2611;
    (t2703, t2706, t2710)
}
