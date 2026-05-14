//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1056/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1056<F: Float>(t6071: F, t886: F, t2770: F, t10673: F, t14675: F, t14690: F, t14703: F, t14705: F, t14712: F, t14715: F, t14716: F, t14722: F, t14726: F, t14730: F, t14734: F, t14494: F, t6035: F) -> (F, F, F) {
    let t18323 = t6071 * t886;
    let t18324 = t2770 * t18323;
    let t18330 = t14675 - t14690 + t14703 + t14705 + t10673 - 0.11337795902333997111e-1 * t14712 + t14715 + 0.27104001498285508386e-3 * t14716 - t14722 + t14726 - t14730 - t14734;
    let t18333 = t14494 * t6035;
    (t18324, t18330, t18333)
}
