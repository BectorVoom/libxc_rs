//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 755/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk755<F: Float>(t359: F, t41: F, t4818: F, t5046: F, t1184: F, t1817: F, t1175: F, t1800: F, t1170: F, t1176: F, t1797: F, t1166: F, t1805: F, t1180: F, t3338: F, t4823: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5047 = t41 * t359;
    let t5048 = t5047 * t4818;
    let t5049 = t5046 * t5048;
    let t5051 = t1184 * t1817;
    let t5053 = t1175 * t1800;
    let t5054 = t1170 * t5053;
    let t5056 = t1797 * t1176;
    let t5058 = t1166 * t1805;
    let t5060 = t1797 * t1180;
    let t5062 = t3338 * t4823;
    (t5047, t5048, t5049, t5051, t5053, t5054, t5056, t5058, t5060, t5062)
}
