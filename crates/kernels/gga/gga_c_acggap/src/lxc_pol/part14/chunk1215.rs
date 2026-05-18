//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1215/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1215<F: Float>(t8998: F, t9076: F, t1938: F, t7973: F, t2131: F, t2132: F, t309: F, t9767: F, t1659: F, t2127: F, t2351: F, t32061: F, t32073: F, t33771: F, t33783: F, t33786: F, t33789: F, t38827: F, t6558: F, t7912: F, t7932: F, t8400: F, t9010: F, t9058: F, t9790: F) -> F {
    let t40793 = t8998 * t9076;
    let t40796 = t7973 * t1938;
    let t40803 = t2131 * t2132 * t9767 * t309;
    let t40815 = -F::new(0.34694512752820797848e1) * t40793 + F::new(0.26341796731742046394e1) * t33771 - F::new(0.65854491829355115987e0) * t40796 - F::new(0.8673628188205199462e0) * t9058 * t2351 + F::new(0.52041769129231196772e1) * t32061 - F::new(0.8673628188205199462e0) * t40803 - F::new(0.13170898365871023197e1) * t9010 * t1659 + t33783 - t32073 + F::new(0.17347256376410398924e1) * t7912 * t9790 - F::new(0.65854491829355115987e0) * t2127 * t6558 - t33786 + F::new(0.17347256376410398924e1) * t33789 + F::new(0.8673628188205199462e0) * t8400 * t7932 * t38827;
    t40815
}
