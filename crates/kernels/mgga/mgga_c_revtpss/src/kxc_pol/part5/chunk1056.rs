//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1056/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1056<F: Float>(t1412: F, t1882: F, t3938: F, t3992: F, t2661: F, t1399: F, t5608: F, t5651: F, t5774: F, t72: F, t686: F, t3915: F) -> (F, F, F, F, F) {
    let t14045 = t1412 * t1882;
    let t14046 = t14045 * t3938;
    let t14047 = t3992 * t14046;
    let t14049 = F::new(0.57165357490759649296e-4) * t2661 * t14047;
    let t14050 = t5608 * t1399;
    let t14051 = t3992 * t14050;
    let t14053 = F::new(0.14291339372689912324e-4) * t2661 * t14051;
    let t14054 = t5651 * t1399;
    let t14055 = t3992 * t14054;
    let t14057 = F::new(0.57165357490759649296e-4) * t2661 * t14055;
    let t14078 = t5774 * t72;
    let t14079 = t14078 * t686;
    let t14081 = F::new(0.19514881078765566038e-1) * t3915 * t14079;
    (t14045, t14049, t14053, t14057, t14081)
}
