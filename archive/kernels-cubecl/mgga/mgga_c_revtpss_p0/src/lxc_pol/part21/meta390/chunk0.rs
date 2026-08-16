//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1835/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1835<F: Float>(t12732: F, t1287: F, t487: F, t12646: F, t1280: F, t1269: F, t3588: F, t1204: F, t3781: F, t1214: F, t1209: F, t5462: F) -> (F, F, F, F, F, F) {
    let t12734 = t487 * t12732 * t1287;
    let t12737 = t1280 * t12646;
    let t12741 = t1269 * t3588 * t1287;
    let t12744 = t1204 * t3781;
    let t12747 = t1214 * t3588;
    let t12748 = t12747 * t1287;
    let t12751 = t1209 * t5462;
    (t12734, t12737, t12741, t12744, t12748, t12751)
}
