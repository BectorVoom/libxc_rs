//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1062/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1062<F: Float>(t1945: F, t9086: F, t5310: F, t8972: F, t17976: F, t7333: F, t1954: F, t9062: F, t4265: F, t8999: F, t1646: F, t2543: F, t18005: F, t18031: F, t18054: F, t18057: F, t1883: F, t1888: F, t1909: F, t22526: F, t22534: F, t22633: F, t22642: F, t22923: F, t22938: F, t2517: F, t5231: F, t7030: F, t725: F, t7340: F, t7349: F, t7360: F, t8931: F, t8975: F) -> (F, F, F, F, F) {
    let t24280 = t1945 * t9086;
    let t24282 = t5310 * t8972;
    let t24284 = t17976 * t7333;
    let t24286 = t9062 * t1954;
    let t24299 = t4265 * t8999;
    let t24304 = t2543 * t1646;
    let t24315 = -0.619125e-2 * t1909 * t8931 - 0.619125e-2 * t725 * t22633 + 0.9286875e-2 * t8975 * t1883 + 0.17687407407407407407e-1 * t18005 - t18031 - 0.1857375e-1 * t5231 * t22534 + 0.46434375e-2 * t7349 * t22923 - 0.35374814814814814815e-1 * t24299 + 0.70749629629629629628e-1 * t18054 - t18057 + 0.24765e-1 * t7360 * t22642 + 0.9286875e-2 * t24304 * t7030 - 0.46434375e-2 * t7349 * t22938 + 0.9286875e-2 * t7349 * t22526 - 0.619125e-2 * t8975 * t1888 + 0.1857375e-1 * t7340 * t2517;
    (t24280, t24282, t24284, t24286, t24315)
}
