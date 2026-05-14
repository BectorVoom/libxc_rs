//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1129/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1129<F: Float>(t25754: F, t420: F, t61889: F, t22534: F, t22572: F, t29494: F, t373: F, t384: F, t4417: F, t100801: F, t100808: F, t100848: F, t115436: F, t2035: F, t22568: F, t22583: F, t22736: F, t22738: F, t25753: F, t29469: F, t29474: F, t29486: F, t4446: F, t4491: F, t5569: F, t5790: F, t58185: F, t73: F, t7867: F, t92482: F, t92666: F, t92710: F, t92770: F, t92957: F) -> (F,) {
    let t115742 = t25754 * t420 * t61889;
    let t115752 = t22534 * t22572 * t29494;
    let t115763 = t4417 * t373 * t384;
    let t115773 = -0.10205883805138882776e-7 * t22736 * t92710 * t29469 - 0.12255510004984495842e-6 * t22736 * t22738 * t29474 + 0.32054706583615839487e-5 * t58185 * t92770 - 0.51789017496114396277e-5 * t25753 * t115742 + 0.11877414311451622578e-3 * t5569 * t22568 * t29486 + 0.39564085156429117904e-4 * t22534 * t22568 * t29494 - 0.4945510644553639738e-5 * t115752 + 0.28374937057613168724e-2 * t100801 + t100808 + 0.85124811172839506173e-2 * t100848 + 0.52700762016626893448e-4 * t7867 * t2035 * t5790 * t4491 + 0.46509801892875584e-1 * t92666 * t4446 + 0.98978452595430188147e-4 * t22583 * t92957 * t115763 - 0.14846767889314528222e-3 * t22583 * t92482 * t115763 - 0.14836531933660919214e-4 * t22534 * t73 * t115436;
    (t115773,)
}
