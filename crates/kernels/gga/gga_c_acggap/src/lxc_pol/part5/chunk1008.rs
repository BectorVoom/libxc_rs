//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1008/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1008<F: Float>(t6021: F, t691: F, t288: F, t5474: F, t75: F, t682: F, t11945: F, t1708: F, t4: F, t657: F, t12157: F, t12665: F, t12673: F, t11954: F, t12126: F, t12130: F, t12148: F, t12156: F, t12661: F, t12664: F, t12669: F, t12672: F, t12677: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20042 = t6021 * t691;
    let t20043 = 0.17315859105681463759e2 * t20042;
    let t20045 = t5474 * t75 * t288;
    let t20046 = 0.11696447245269292414e1 * t20045;
    let t20047 = t6021 * t682;
    let t20048 = 0.5848223622634646207e0 * t20047;
    let t20049 = 0.20508037716432813316e4 * t11945;
    let t20051 = t1708 * t4 * t657;
    let t20052 = 0.10843581300301739842e-1 * t20051;
    let t20053 = 24.0 * t12157;
    let t20054 = 0.65061487801810439052e-1 * t12665;
    let t20055 = 0.96319466275353142156e0 * t12673;
    let t20056 = -t20043 - t20046 - t20048 - t20049 - t11954 + t20052 + t12148 + t12156 - t20053 - t12661 - t12664 - t20054 - t12669 + t12672 + t20055 + t12677 - t12126 + t12130;
    (t20043, t20046, t20048, t20049, t20052, t20053, t20054, t20055, t20056)
}
