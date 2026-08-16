//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 889/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk889(t356: f64, t638: f64, t639: f64, t9745: f64, t34705: f64, t34707: f64, t34711: f64, t38676: f64, t38705: f64, t38710: f64, t38712: f64, t44886: f64, t44888: f64, t44891: f64, t44894: f64, t44901: f64, t44906: f64, t44909: f64, t44911: f64, t44916: f64) -> f64 {
    let t44920 = t638 * t639 * t9745 * t356;
    let t44922 = 0.12414674968878536491e-4_f64 * t44886 - 0.19863479950205658386e-4_f64 * t44888 - t38676 + t34705 + t34707 - t34711 + 0.72042316457491791906e-3_f64 * t44891 - 0.10248087766267884742e-3_f64 * t44894 + t38705 - 0.23836175940246790063e-3_f64 * t38710 - 0.59590439850616975157e-4_f64 * t38712 - 0.31923449919973379548e-4_f64 * t44901 - 0.31923449919973379548e-4_f64 * t44906 - 0.99317399751028291929e-5_f64 * t44909 - 0.27274661654245341728e-1_f64 * t44911 + 0.15243824895787514157e-3_f64 * t44916 + 0.15243824895787514157e-3_f64 * t44920;
    t44922
}
