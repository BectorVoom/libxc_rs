//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1023/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1023<F: Float>(t2434: F, t371: F, t373: F, t367: F, t1065: F, t675: F, t247: F, t906: F, t1063: F, t1062: F, t3223: F, t1052: F, t3147: F) -> (F, F, F, F, F) {
    let t11970 = t371 * t2434 * t373;
    let t11972 = F::new(0.63517063878621832551e-4) * t367 * t11970;
    let t11986 = t675 * t1065;
    let t11988 = t247 * t11986 * t906;
    let t11989 = t1063 * t11988;
    let t11994 = t3223 * t1062;
    let t11997 = t1052 * t3147;
    (t11972, t11986, t11989, t11994, t11997)
}
