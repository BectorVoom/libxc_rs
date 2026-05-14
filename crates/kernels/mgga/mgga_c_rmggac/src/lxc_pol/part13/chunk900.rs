//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 900/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk900<F: Float>(t9097: F, t9107: F, t9112: F, t9114: F, t9119: F, t7908: F, t7910: F, t8223: F, t8224: F, t8293: F, t8295: F, t8297: F, t9124: F, t9637: F, t34659: F, t34662: F, t34665: F, t38312: F, t38314: F, t38318: F, t38322: F, t38326: F, t38328: F, t38336: F, t38340: F, t38344: F, t38348: F, t38352: F, t38356: F, t38358: F, t38361: F) -> (F, F, F, F, F) {
    let t42563 = 0.5107751987195740728e-4 * t9097;
    let t42567 = 0.5107751987195740728e-4 * t9107;
    let t42568 = 0.1702583995731913576e-4 * t9112;
    let t42569 = 0.1702583995731913576e-4 * t9114;
    let t42570 = 0.638468998399467591e-4 * t9119;
    let t42571 = 0.79453919800822633544e-4 * t7908 - 0.23836175940246790064e-3 * t7910 - t8223 + t8224 + t8293 + t8295 + t8297 + t42567 + t42568 - t42569 - t42570;
    let t42574 = 0.212822999466489197e-4 * t9124;
    let t42575 = 0.79828278012425390428e-1 * t9637;
    let t42593 = 0.162600798888400151e-2 * t38312 + 0.3842256877732895568e-2 * t38314 - 0.66671395154821946452e-1 * t38318 + 0.1333427903096438929e0 * t34659 - 0.39032073591371545778e-3 * t38322 + 0.60975299583150056624e-3 * t38326 - 0.11974241701863808564e0 * t38328 + 0.59620292925746722032e-2 * t34662 + 0.59620292925746722032e-2 * t34665 + 0.30487649791575028312e-3 * t38336 + 0.30487649791575028312e-3 * t38340 + 0.60975299583150056624e-3 * t38344 + 0.30487649791575028312e-3 * t38348 - 0.1702583995731913576e-4 * t38352 - 0.1702583995731913576e-4 * t38356 - 0.85129199786595678799e-5 * t38358 + 0.5107751987195740728e-4 * t38361;
    (t42563, t42571, t42574, t42575, t42593)
}
