//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1046/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1046<F: Float>(t13726: F, t2439: F, t1903: F, t4075: F, t1444: F, t556: F, t2782: F, t212: F, t5710: F, t1358: F, t689: F, t221: F, t3979: F, t5591: F) -> (F, F, F, F) {
    let t13727 = t2439 * t13726;
    let t13729 = t4075 * t1903;
    let t13730 = t13729 * t1444;
    let t13731 = t556 * t13730;
    let t13733 = F::cast_from(0.21951497276451705328e-1_f64) * t2782 * t13731;
    let t13734 = t212 * t5710;
    let t13735 = t13734 * t1358;
    let t13737 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t13735;
    let t13760 = t3979 * t221 * t5591;
    (t13727, t13733, t13737, t13760)
}
