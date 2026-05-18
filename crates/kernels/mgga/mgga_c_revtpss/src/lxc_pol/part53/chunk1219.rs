//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1219/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1219<F: Float>(t28189: F, t8764: F, t32855: F, t7732: F, t2322: F, t34382: F, t125491: F, t125495: F, t125497: F, t125499: F, t125500: F, t125502: F, t125505: F, t125507: F, t125510: F, t34394: F, t649: F) -> F {
    let t129322 = t8764 * t28189;
    let t129326 = t7732 * t32855;
    let t129328 = t2322 * t34382;
    let t129330 = -t34394 * t649 - t125491 + t125495 + F::new(6.0) * t125497 - t125499 - t125500 + F::new(3.0) * t125502 - t125505 - t125507 + t125510 - t129322 - F::new(2.0) * t129326 - F::new(2.0) * t129328;
    t129330
}
