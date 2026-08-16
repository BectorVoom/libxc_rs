//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1339/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1339(t2402: f64, t976: f64, t973: f64, t979: f64, t2955: f64, t2967: f64, t10209: f64, t10217: f64, t10263: f64, t10325: f64, t2960: f64, t2979: f64, t3000: f64, t343: f64, t39097: f64, t42554: f64, t42861: f64, t42862: f64, t42873: f64, t42877: f64, t42889: f64, t4546: f64, t980: f64, t984: f64, t987: f64) -> f64 {
    let t42891 = t2402 * t976;
    let t42893 = t973 * t42891 * t979;
    let t42895 = t2955 * t2967;
    let t42899 = 0.28806584362139917695e-2_f64 * t973 * t42861 * t42862 * t39097 - 0.33333333333333333332e-2_f64 * t973 * t4546 * t10325 * t984 * t343 - 0.37037037037037037036e-3_f64 * t42873 - 0.49382716049382716048e-3_f64 * t42877 + 0.16296296296296296296e-1_f64 * t10263 * t3000 + 0.26666666666666666666e-1_f64 * t2960 * t10209 + 0.13333333333333333332e-1_f64 * t973 * t2979 * t10217 * t39097 - 0.50699588477366255142e-1_f64 * t42554 * t980 + 0.1086419753086419753e-1_f64 * t42889 + 0.41152263374485596707e-3_f64 * t42893 - 0.1086419753086419753e-1_f64 * t42895 + 0.15209876543209876543e0_f64 * t42554 * t987;
    t42899
}
