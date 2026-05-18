//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 908/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk908<F: Float>(t30080: F, t9948: F, t3928: F, t6449: F, t645: F, t6434: F, t5016: F, t9951: F, t9128: F, t1550: F, t2060: F, t30344: F) -> (F, F, F, F, F, F) {
    let t45209 = t30080 * t9948;
    let t45212 = t3928 * t645 * t6449;
    let t45215 = t3928 * t645 * t6434;
    let t45217 = t5016 * t9951;
    let t45219 = t9128 * t9951;
    let t45222 = t1550 * t2060 * t30344;
    (t45209, t45212, t45215, t45217, t45219, t45222)
}
