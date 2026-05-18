//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 763/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk763<F: Float>(t4210: F, t7932: F, t7942: F, t609: F, t862: F, t865: F, t1265: F, t2127: F, t2143: F, t2146: F, t2155: F, t2159: F, t616: F, t621: F, t7307: F, t7879: F, t7889: F, t7893: F, t7900: F, t7901: F, t7904: F, t7909: F, t7912: F, t7917: F, t7921: F, t7926: F, t7929: F, t7931: F, t7935: F, t7938: F) -> (F, F, F, F, F) {
    let t7943 = t7932 * t4210;
    let t7944 = t7942 * t7943;
    let t7948 = t862 * t609;
    let t7950 = F::new(0.13170898365871023197e1) * t7948 * t865;
    let t7951 = F::new(0.34694512752820797848e1) * t7307 - F::new(0.4336814094102599731e0) * t616 * t7879 - F::new(0.8673628188205199462e0) * t2143 * t2159 - t7889 - F::new(0.8673628188205199462e0) * t2146 * t7893 + t7900 + F::new(0.13170898365871023197e1) * t7901 + F::new(0.4336814094102599731e0) * t2146 * t7904 - F::new(0.34694512752820797848e1) * t7909 + F::new(0.8673628188205199462e0) * t7912 * t2155 + F::new(0.8673628188205199462e0) * t2146 * t7917 - t7921 - t7926 - t7929 - F::new(0.17347256376410398924e1) * t7931 * t7935 - F::new(0.4336814094102599731e0) * t7938 * t621 - F::new(0.17347256376410398924e1) * t7944 - F::new(0.65854491829355115987e0) * t2127 * t1265 + t7950;
    (t7943, t7944, t7948, t7950, t7951)
}
