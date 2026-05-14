//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 833/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk833<F: Float>(t14657: F, t14683: F, t14655: F, t14662: F, t14666: F, t14669: F, t14673: F, t14676: F, t14680: F, t14688: F, t14692: F, t14715: F, t10279: F, t10400: F, t10552: F, t10555: F, t10636: F, t10641: F, t10643: F, t14697: F, t14701: F, t14706: F) -> (F, F) {
    let t14929 = 2.0 / 9.0 * t14657;
    let t14936 = 4.0 / 3.0 * t14683;
    let t14939 = 2.0 / 9.0 * t14655 - t14929 + 2.0 / 3.0 * t14662 + t14666 / 3.0 + 4.0 / 3.0 * t14669 - 2.0 / 3.0 * t14673 - 2.0 * t14676 - 4.0 / 3.0 * t14680 - t14936 + 4.0 / 9.0 * t14688 - 4.0 / 3.0 * t14692;
    let t14946 = 4.0 / 27.0 * t14715;
    let t14947 = 4.0 * t14697 + 2.0 * t14701 - 6.0 * t14706 + t10552 - 8.0 / 9.0 * t10400 - t10555 - t10636 - 8.0 / 27.0 * t10279 + t10641 + t10643 - t14946;
    (t14939, t14947)
}
