//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 885/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk885<F: Float>(t32003: F, t33535: F, t4210: F, t32130: F, t7965: F, t2131: F, t2147: F, t309: F, t8392: F, t1411: F, t463: F, t7932: F, t7963: F, t23688: F, t7942: F, t31895: F, t31897: F, t31901: F, t31905: F, t33518: F, t33523: F, t33525: F, t33529: F, t33533: F, t7931: F) -> (F,) {
    let t33538 = 0.34694512752820797848e1 * t32003 * t33535 * t4210;
    let t33541 = 0.34694512752820797848e1 * t32130 * t33535 * t7965;
    let t33546 = 0.34694512752820797848e1 * t2131 * t2147 * t8392 * t309;
    let t33547 = t1411 * t463;
    let t33551 = t1411 * t309;
    let t33554 = 0.17347256376410398924e1 * t7963 * t7932 * t33551;
    let t33557 = 0.17347256376410398924e1 * t7942 * t7932 * t23688;
    let t33558 = -0.8673628188205199462e0 * t33518 + t33523 - 0.8673628188205199462e0 * t33525 - t33529 - 0.8673628188205199462e0 * t31895 - 0.17347256376410398924e1 * t31897 + t33533 - 0.17347256376410398924e1 * t31901 + t33538 - t33541 - 0.34694512752820797848e1 * t31905 + t33546 - 0.17347256376410398924e1 * t7931 * t7932 * t33547 + t33554 - t33557;
    (t33558,)
}
