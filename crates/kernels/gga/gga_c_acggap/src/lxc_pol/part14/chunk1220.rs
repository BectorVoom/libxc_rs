//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1220/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1220<F: Float>(t1915: F, t7973: F, t2131: F, t309: F, t8004: F, t9497: F, t2138: F, t322: F, t157: F, t1937: F, t2122: F, t2146: F, t2147: F, t2152: F, t32222: F, t33778: F, t36526: F, t36531: F, t36533: F, t36541: F, t36543: F, t36555: F, t406: F, t7912: F, t8393: F, t9003: F, t9026: F, t9493: F, t9767: F) -> F {
    let t40909 = t7973 * t1915;
    let t40918 = t2131 * t8004 * t9497 * t309;
    let t40922 = t2138 * t8004 * t9497 * t322;
    let t40939 = -F::new(0.52041769129231196772e1) * t36526 + F::new(0.13170898365871023197e1) * t40909 + F::new(0.8673628188205199462e0) * t2146 * t2147 * t2122 * t1937 + F::new(0.13170898365871023197e1) * t36531 - F::new(0.52041769129231196772e1) * t40918 + F::new(0.52041769129231196772e1) * t40922 + F::new(0.34694512752820797848e1) * t36533 + F::new(0.4336814094102599731e0) * t2146 * t2152 * t9767 * t406 * t157 - F::new(0.17347256376410398924e1) * t36541 + F::new(0.17347256376410398924e1) * t36543 + F::new(0.17347256376410398924e1) * t9003 * t8393 + F::new(0.52041769129231196772e1) * t36555 - F::new(0.8673628188205199462e0) * t7912 * t9493 + t32222 - F::new(0.17347256376410398924e1) * t33778 * t9026;
    t40939
}
