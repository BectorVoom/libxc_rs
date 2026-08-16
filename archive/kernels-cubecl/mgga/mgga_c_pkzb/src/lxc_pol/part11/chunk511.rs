//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 511/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk511<F: Float>(t2609: F, t557: F, t1501: F, t1510: F, t1520: F, t1530: F, t1534: F, t1544: F, t1547: F, t1553: F, t2535: F, t2559: F, t2606: F, t2608: F) -> (F, F, F) {
    let t2610 = t2609 * t557;
    let t2611 = F::cast_from(0.5848223622634646207e0_f64) * t2610;
    let t2612 = -t1501 - t1510 - t2535 - t1520 + t1530 + t1534 + t2559 + t2606 + t2608 + t1544 - t1547 - t2611 - t1553;
    (t2610, t2611, t2612)
}
