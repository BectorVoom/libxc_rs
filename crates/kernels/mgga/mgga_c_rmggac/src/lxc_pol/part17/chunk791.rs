//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 791/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk791<F: Float>(t10044: F, t1982: F, t7428: F, t8365: F, t8562: F, t131: F, t6344: F, t638: F, t639: F, t71: F, t356: F, t9745: F, t34705: F, t34707: F, t34711: F, t38676: F, t38705: F, t38710: F, t38712: F, t44886: F, t44888: F, t44891: F, t44894: F, t44901: F, t44906: F) -> (F,) {
    let t44909 = t10044 * t7428 * t1982;
    let t44911 = t8365 * t8562;
    let t44916 = t638 * t639 * t71 * t6344 * t131;
    let t44920 = t638 * t639 * t9745 * t356;
    let t44922 = 0.12414674968878536491e-4 * t44886 - 0.19863479950205658386e-4 * t44888 - t38676 + t34705 + t34707 - t34711 + 0.72042316457491791906e-3 * t44891 - 0.10248087766267884742e-3 * t44894 + t38705 - 0.23836175940246790063e-3 * t38710 - 0.59590439850616975157e-4 * t38712 - 0.31923449919973379548e-4 * t44901 - 0.31923449919973379548e-4 * t44906 - 0.99317399751028291929e-5 * t44909 - 0.27274661654245341728e-1 * t44911 + 0.15243824895787514157e-3 * t44916 + 0.15243824895787514157e-3 * t44920;
    (t44922,)
}
