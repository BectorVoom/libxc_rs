//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1250/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1250<F: Float>(t33153: F, t10627: F, t1835: F, t7572: F, t7573: F, t10914: F, t10915: F, t32897: F, t25198: F, t7391: F, t3487: F, t739: F, t7803: F, t7805: F) -> (F, F, F, F, F, F) {
    let t33154 = F::cast_from(0.51762950037793012063e1_f64) * t33153;
    let t33155 = t10627 * t1835;
    let t33158 = F::cast_from(0.69017266717057349418e1_f64) * t7572 * t7573 * t33155;
    let t33164 = F::cast_from(0.42900587942220512002e1_f64) * t10914 * t10915 * t32897;
    let t33178 = t25198 * t7391;
    let t33179 = F::cast_from(0.89376224879626066674e-1_f64) * t33178;
    let t33182 = t7803 * t739 * t3487 * t7805;
    (t33154, t33155, t33158, t33164, t33179, t33182)
}
