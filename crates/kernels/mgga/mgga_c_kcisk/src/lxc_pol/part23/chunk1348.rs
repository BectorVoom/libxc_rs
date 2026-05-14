//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1348/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1348<F: Float>(t113642: F, t109417: F, t110256: F, t110261: F, t110294: F, t110341: F, t113612: F, t113615: F, t113620: F, t113622: F, t113629: F, t113636: F, t113641: F, t32035: F, t32096: F, t33400: F, t33460: F, t9426: F, t9446: F, t9805: F) -> (F,) {
    let t113643 = 0.22109259259259259258e-2 * t113642;
    let t113644 = -0.120625e-1 * t33460 * t32035 + 0.14739506172839506172e-2 * t109417 - 0.16581944444444444444e-2 * t113612 - 0.66327777777777777776e-2 * t113615 - 0.46296296296296296298e-2 * t110256 - 0.69444444444444444446e-2 * t110261 - t113620 - t113622 + 0.18518518518518518519e-1 * t110341 * t9805 + 0.18518518518518518519e-1 * t110294 * t9805 + 0.44229166666666666667e-1 * t9426 * t113629 - 0.20833333333333333334e-1 * t32096 * t33400 - 0.10416666666666666667e-1 * t9446 * t113636 - t113641 - t113643;
    (t113644,)
}
