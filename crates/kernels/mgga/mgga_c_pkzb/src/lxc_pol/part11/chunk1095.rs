//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1095/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1095<F: Float>(t19758: F, t1625: F, t2557: F, t83: F, t1008: F, t5075: F, t1548: F, t2607: F, t1009: F, t4882: F, t5137: F, t1508: F, t7035: F) -> (F, F, F, F, F, F, F) {
    let t19759 = F::new(0.35089341735807877242e1) * t19758;
    let t19775 = t83 * t2557 * t1625;
    let t19776 = F::new(3.0) * t19775;
    let t19778 = t83 * t1008 * t5075;
    let t19797 = t1548 * t2607;
    let t19798 = F::new(96.0) * t19797;
    let t19803 = t4882 * t1009;
    let t19804 = F::new(240.0) * t19803;
    let t19805 = t5137 * t1009;
    let t19822 = t7035 * t1508;
    (t19759, t19776, t19778, t19798, t19804, t19805, t19822)
}
