//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 712/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk712<F: Float>(t12671: F, t3260: F, t3232: F, t981: F, t1036: F, t1039: F, t3139: F, t3241: F, t212: F, t916: F, t211: F, t210: F) -> (F, F, F, F) {
    let t12672 = t12671 * t3260;
    let t12674 = t3232 * t981;
    let t12675 = t12674 * t1036;
    let t12677 = t1039 * t3139;
    let t12678 = t3241 * t12677;
    let t12680 = t212 * t916;
    let t12681 = F::new(1.0) / t12680;
    let t12682 = t211 * t12681;
    let t12683 = t210 * t12682;
    (t12672, t12675, t12678, t12683)
}
