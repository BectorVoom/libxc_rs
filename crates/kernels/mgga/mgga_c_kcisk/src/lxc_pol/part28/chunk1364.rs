//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1364/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1364<F: Float>(t116201: F, t116599: F, t116601: F, t116620: F, t116623: F, t116625: F, t116790: F, t116793: F, t117086: F, t121236: F, t121284: F, t121304: F, t2781: F, t2785: F, t34122: F, t34218: F, t34225: F, t34261: F, t71223: F, t9649: F, t9664: F, t9672: F, t9922: F, t9940: F) -> (F,) {
    let t121435 = -0.20833333333333333334e-1 * t9664 * t121304 - 0.23148148148148148149e-2 * t116599 - 0.80416666666666666668e-2 * t116601 + 0.120625e-1 * t9649 * t121236 - t116620 - t116623 - t116625 + 0.20833333333333333334e-1 * t116790 * t9940 + 0.20833333333333333334e-1 * t116201 * t9940 + 0.20833333333333333334e-1 * t34122 * t34261 - 0.10416666666666666667e-1 * t71223 * t2781 * t2785 - 0.21444444444444444446e-1 * t116793 * t9922 - 0.21444444444444444446e-1 * t117086 * t9922 - 0.21444444444444444446e-1 * t34225 * t34218 + 0.20833333333333333334e-1 * t121284 * t9672;
    (t121435,)
}
