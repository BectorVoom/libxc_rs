//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1077/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1077<F: Float>(t11847: F, t141: F, t1515: F, t2202: F, t1509: F, t9267: F, t2869: F, t9271: F, t2868: F, t4079: F, t1027: F, t2877: F, t4071: F) -> (F, F, F, F, F, F) {
    let t11848 = t141 * t11847;
    let t11850 = t2202 * t1515;
    let t11852 = t9267 * t1509;
    let t11853 = t11852 * t2869;
    let t11856 = t9271 * t1509;
    let t11857 = t11856 * t2869;
    let t11859 = t2868 * t4079;
    let t11860 = t11859 * t1027;
    let t11862 = t4071 * t2877;
    (t11848, t11850, t11853, t11857, t11860, t11862)
}
