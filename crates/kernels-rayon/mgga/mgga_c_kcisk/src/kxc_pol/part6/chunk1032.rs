//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1032/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1032(t30899: f64, t30946: f64, t416: f64, t467: f64, t471: f64, t415: f64, t1220: f64, t13400: f64, t19788: f64, t26764: f64, t26785: f64, t26787: f64, t26841: f64, t30236: f64, t30241: f64, t30244: f64, t30247: f64, t30254: f64, t30258: f64, t30262: f64, t30264: f64, t30266: f64, t30771: f64, t30775: f64, t3930: f64) -> (f64, f64, f64) {
    let t30947 = t30899 + t30946;
    let t30948 = t416 * t30947;
    let t30949 = t30948 * t467;
    let t30950 = t30949 * t471;
    let t30951 = t415 * t30950;
    let t30953 = 0.73697530864197530861e-2_f64 * t30236 + 0.16581944444444444444e-2_f64 * t30241 + 0.49745833333333333332e-2_f64 * t30244 + 0.49745833333333333332e-2_f64 * t30247 - 0.66327777777777777775e-2_f64 * t26764 + 0.66327777777777777776e-2_f64 * t26785 + 0.33163888888888888887e-2_f64 * t26787 + 0.99491666666666666664e-2_f64 * t30254 + t13400 + 0.33163888888888888887e-2_f64 * t30258 - 0.66327777777777777776e-2_f64 * t30262 - 0.99491666666666666664e-2_f64 * t30264 + 0.66327777777777777776e-2_f64 * t30266 - 0.66327777777777777776e-2_f64 * t26841 - 0.193e0_f64 * t1220 * t30771 - 0.223494e0_f64 * t3930 * t30775 - 0.16581944444444444444e-2_f64 * t19788 + 0.24872916666666666666e-2_f64 * t30951;
    (t30947, t30951, t30953)
}
