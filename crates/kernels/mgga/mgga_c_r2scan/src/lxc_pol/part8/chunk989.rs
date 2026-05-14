//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 989/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk989<F: Float>(t2858: F, t9583: F, t2526: F, t2859: F, t2333: F, t3245: F, t2266: F, t481: F, t2900: F, t6621: F, t806: F, t35: F, t990: F, t1216: F, t1248: F, t2904: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9584 = t2858 * t9583;
    let t9585 = 6.0 * t9584;
    let t9586 = t2859 * t2526;
    let t9587 = t2858 * t9586;
    let t9588 = 12.0 * t9587;
    let t9589 = t3245 * t2333;
    let t9591 = t2266 * t9589 * t481;
    let t9592 = 3.0 * t9591;
    let t9597 = t6621 * t2900;
    let t9598 = t9597 * t806;
    let t9601 = t990 * t35;
    let t9602 = t9601 * t1216;
    let t9607 = t1248 * t2904;
    (t9584, t9585, t9587, t9588, t9589, t9591, t9592, t9597, t9598, t9602, t9607)
}
