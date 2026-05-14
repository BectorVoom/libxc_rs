//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1092/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1092<F: Float>(t108738: F, t108739: F, t108754: F, t108758: F, t108761: F, t108766: F, t108773: F, t108781: F, t13395: F, t17807: F, t231: F, t232: F, t24265: F, t24306: F, t24361: F, t27500: F, t27601: F, t27605: F, t6035: F, t65689: F, t65694: F, t65699: F, t65763: F, t66066: F, t66071: F, t684: F, t96462: F, t96479: F, t96615: F, t96750: F) -> (F,) {
    let t108782 = 0.74233839446572641111e-4 * t96462 - 0.14846767889314528222e-4 * t96479 + 0.2071560699844575851e-4 * t108738 * t96615 * t231 * t108739 - 0.89080607335887169332e-3 * t24265 * t232 * t65763 - 0.44540303667943584666e-3 * t24265 * t232 * t66066 + 0.29673063867321838427e-4 * t96750 * t232 * t66071 - 0.85124811172839506173e-2 * t27500 * t108754 - 0.19862455940329218107e-1 * t27500 * t108758 + 0.25537443351851851852e-1 * t24361 * t6035 * t108761 * t684 + 0.60102574844279699039e-6 * t24306 * t108766 + 0.36061544906567819424e-6 * t65699 * t27601 + 0.30030568862539529421e-7 * t65689 * t27601 + 0.32054706583615839486e-5 * t65694 * t108773 - 0.12255510004984495842e-6 * t17807 * t27605 * t13395 + t108781;
    (t108782,)
}
