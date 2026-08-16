//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1971/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1971(t86886: f64, t86895: f64, t2053: f64, t40889: f64, t10049: f64, t13049: f64, t25168: f64, t26713: f64, t2743: f64, t7842: f64, t866: f64, t86847: f64, t86852: f64, t86857: f64, t86862: f64, t86866: f64, t86875: f64, t86881: f64, t86884: f64, t86891: f64, t86901: f64, t86903: f64, t92375: f64, t92382: f64, t92383: f64, t92386: f64) -> f64 {
    let t92390 = 0.15352717957250113407e0_f64 * t86886;
    let t92393 = 0.3289868133696452873e-1_f64 * t86895;
    let t92394 = t40889 * t2053;
    let t92400 = t92375 + 0.3289868133696452873e-1_f64 * t86847 - t26713 * t2743 + 0.6579736267392905746e-1_f64 * t86852 + 0.6579736267392905746e-1_f64 * t86857 + 0.6579736267392905746e-1_f64 * t86862 + 0.3289868133696452873e-1_f64 * t86866 + t92382 - t92383 + 0.6579736267392905746e-1_f64 * t86875 - 0.9869604401089358619e-1_f64 * t86881 - 2.0_f64 * t92386 * t866 + 0.6579736267392905746e-1_f64 * t86884 + t92390 - t10049 * t7842 - 0.3289868133696452873e-1_f64 * t86891 + t92393 + 24.0_f64 * t25168 * t92394 * t13049 + 0.9869604401089358619e-1_f64 * t86901 - 0.25587863262083522346e0_f64 * t86903;
    t92400
}
