//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1220/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1220(t1915: f64, t7973: f64, t2131: f64, t309: f64, t8004: f64, t9497: f64, t2138: f64, t322: f64, t157: f64, t1937: f64, t2122: f64, t2146: f64, t2147: f64, t2152: f64, t32222: f64, t33778: f64, t36526: f64, t36531: f64, t36533: f64, t36541: f64, t36543: f64, t36555: f64, t406: f64, t7912: f64, t8393: f64, t9003: f64, t9026: f64, t9493: f64, t9767: f64) -> f64 {
    let t40909 = t7973 * t1915;
    let t40918 = t2131 * t8004 * t9497 * t309;
    let t40922 = t2138 * t8004 * t9497 * t322;
    let t40939 = -0.52041769129231196772e1_f64 * t36526 + 0.13170898365871023197e1_f64 * t40909 + 0.8673628188205199462e0_f64 * t2146 * t2147 * t2122 * t1937 + 0.13170898365871023197e1_f64 * t36531 - 0.52041769129231196772e1_f64 * t40918 + 0.52041769129231196772e1_f64 * t40922 + 0.34694512752820797848e1_f64 * t36533 + 0.4336814094102599731e0_f64 * t2146 * t2152 * t9767 * t406 * t157 - 0.17347256376410398924e1_f64 * t36541 + 0.17347256376410398924e1_f64 * t36543 + 0.17347256376410398924e1_f64 * t9003 * t8393 + 0.52041769129231196772e1_f64 * t36555 - 0.8673628188205199462e0_f64 * t7912 * t9493 + t32222 - 0.17347256376410398924e1_f64 * t33778 * t9026;
    t40939
}
