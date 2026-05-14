//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1384/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1384<F: Float>(t121860: F, t1895: F, t415: F, t112602: F, t116123: F, t116201: F, t116489: F, t121381: F, t121600: F, t121828: F, t121831: F, t121834: F, t121838: F, t121851: F, t121856: F, t33021: F, t33031: F, t34027: F, t68256: F, t7261: F, t9649: F, t9664: F, t9667: F, t9922: F) -> (F, F) {
    let t121862 = t415 * t121860 * t1895;
    let t121866 = 0.18424382716049382715e-2 * t121828 + 0.73697530864197530861e-2 * t121831 - 0.11054629629629629629e-1 * t121834 - 0.55555555555555555558e-1 * t116123 * t9922 + 0.11054629629629629629e-2 * t121838 + 0.23148148148148148149e-2 * t112602 - 0.71481481481481481487e-2 * t116489 * t34027 + 0.20833333333333333334e-1 * t116201 * t9922 - 0.20833333333333333334e-1 * t9664 * t7261 * t33021 * t68256 - 0.10416666666666666667e-1 * t9664 * t121851 - 0.40208333333333333335e-2 * t9649 * t121851 - 0.23148148148148148149e-2 * t121856 - 0.34722222222222222223e-2 * t121600 * t9667 - 0.24872916666666666666e-2 * t121862 + 0.41666666666666666668e-1 * t33031 * t121381;
    (t121862, t121866)
}
