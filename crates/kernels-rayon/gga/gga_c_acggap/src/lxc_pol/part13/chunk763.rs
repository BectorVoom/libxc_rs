//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 763/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk763(t4210: f64, t7932: f64, t7942: f64, t609: f64, t862: f64, t865: f64, t1265: f64, t2127: f64, t2143: f64, t2146: f64, t2155: f64, t2159: f64, t616: f64, t621: f64, t7307: f64, t7879: f64, t7889: f64, t7893: f64, t7900: f64, t7901: f64, t7904: f64, t7909: f64, t7912: f64, t7917: f64, t7921: f64, t7926: f64, t7929: f64, t7931: f64, t7935: f64, t7938: f64) -> (f64, f64, f64, f64, f64) {
    let t7943 = t7932 * t4210;
    let t7944 = t7942 * t7943;
    let t7948 = t862 * t609;
    let t7950 = 0.13170898365871023197e1_f64 * t7948 * t865;
    let t7951 = 0.34694512752820797848e1_f64 * t7307 - 0.4336814094102599731e0_f64 * t616 * t7879 - 0.8673628188205199462e0_f64 * t2143 * t2159 - t7889 - 0.8673628188205199462e0_f64 * t2146 * t7893 + t7900 + 0.13170898365871023197e1_f64 * t7901 + 0.4336814094102599731e0_f64 * t2146 * t7904 - 0.34694512752820797848e1_f64 * t7909 + 0.8673628188205199462e0_f64 * t7912 * t2155 + 0.8673628188205199462e0_f64 * t2146 * t7917 - t7921 - t7926 - t7929 - 0.17347256376410398924e1_f64 * t7931 * t7935 - 0.4336814094102599731e0_f64 * t7938 * t621 - 0.17347256376410398924e1_f64 * t7944 - 0.65854491829355115987e0_f64 * t2127 * t1265 + t7950;
    (t7943, t7944, t7948, t7950, t7951)
}
