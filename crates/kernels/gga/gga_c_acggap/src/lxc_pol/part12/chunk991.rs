//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 991/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk991<F: Float>(t310: F, t8322: F, t2132: F, t2229: F, t7885: F, t879: F, t2138: F, t2147: F, t463: F, t8060: F, t1222: F, t8331: F) -> (F, F, F, F) {
    let t33294 = t310 * t8322;
    let t33301 = F::new(0.78062653693846795158e1) * t7885 * t2132 * t2229 * t879;
    let t33306 = t2138 * t2147 * t8060 * t463;
    let t33308 = t8331 * t1222;
    (t33294, t33301, t33306, t33308)
}
