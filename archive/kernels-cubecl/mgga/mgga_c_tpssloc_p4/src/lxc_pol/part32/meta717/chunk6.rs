//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2277/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2277<F: Float>(t100578: F, t100623: F, t23788: F, t67128: F, t16949: F, t25891: F, t25927: F, t98102: F, t5966: F, t868: F, t1649: F, t4255: F, t870: F) -> (F, F, F, F, F, F) {
    let t100624 = t100578 + t100623;
    let t100638 = t23788 * t67128;
    let t100641 = t25891 * t16949;
    let t100644 = t25927 * t98102;
    let t100646 = t5966 * t868;
    let t100651 = t870 * t1649 * t4255;
    (t100624, t100638, t100641, t100644, t100646, t100651)
}
