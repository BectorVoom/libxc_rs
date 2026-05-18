//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1117/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1117<F: Float>(t13784: F, t3938: F, t13783: F, t3935: F, t828: F, t1882: F, t4003: F, t1353: F, t1398: F, t3957: F, t5690: F, t1873: F, t9741: F) -> (F, F, F, F, F, F) {
    let t13785 = t13784 * t3938;
    let t13786 = t13783 * t13785;
    let t13789 = t3935 * t828;
    let t13790 = t1882 * t4003;
    let t13791 = t1353 * t1398;
    let t13792 = t13790 * t13791;
    let t13793 = t13789 * t13792;
    let t13797 = F::new(7.0) / F::new(72.0) * t3957 * t5690;
    let t13798 = t9741 * t1873;
    (t13786, t13789, t13790, t13793, t13797, t13798)
}
