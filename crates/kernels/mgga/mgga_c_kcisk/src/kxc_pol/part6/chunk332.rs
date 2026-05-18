//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 332/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk332<F: Float>(t1676: F, t591: F, t240: F, t794: F, t772: F) -> (F, F, F, F, F) {
    let t1979 = t591 * t1676;
    let t1987 = t240 * t591;
    let t1992 = t794 * t794;
    let t1993 = F::new(1.0) / t1992;
    let t1994 = t772 * t1993;
    (t1979, t1987, t1992, t1993, t1994)
}
