//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 770/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk770<F: Float>(t1878: F, t454: F, t142: F, t1809: F, t1832: F, t5504: F, t5519: F, t3268: F, t3276: F, t1697: F, t2610: F, t102: F, t2615: F, t411: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7082 = t454 * t1878;
    let t7083 = t7082 * t142;
    let t7085 = t1809 * t1832;
    let t7093 = F::cast_from(1.2991222222222223_f64) * t5504;
    let t7096 = F::cast_from(0.6495611111111111_f64) * t5519;
    let t7100 = F::cast_from(0.3247805555555556_f64) * t3268;
    let t7101 = F::cast_from(0.6495611111111111_f64) * t3276;
    let t7102 = t1697 * t2610;
    let t7108 = F::cast_from(17.53815_f64) * t102 * t2615 * t411;
    (t7082, t7083, t7085, t7093, t7096, t7100, t7101, t7102, t7108)
}
