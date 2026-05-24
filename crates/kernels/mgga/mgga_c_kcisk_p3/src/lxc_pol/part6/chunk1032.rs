//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1032/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1032<F: Float>(t30899: F, t30946: F, t416: F, t467: F, t471: F, t415: F, t1220: F, t13400: F, t19788: F, t26764: F, t26785: F, t26787: F, t26841: F, t30236: F, t30241: F, t30244: F, t30247: F, t30254: F, t30258: F, t30262: F, t30264: F, t30266: F, t30771: F, t30775: F, t3930: F) -> (F, F, F) {
    let t30947 = t30899 + t30946;
    let t30948 = t416 * t30947;
    let t30949 = t30948 * t467;
    let t30950 = t30949 * t471;
    let t30951 = t415 * t30950;
    let t30953 = F::cast_from(0.73697530864197530861e-2_f64) * t30236 + F::cast_from(0.16581944444444444444e-2_f64) * t30241 + F::cast_from(0.49745833333333333332e-2_f64) * t30244 + F::cast_from(0.49745833333333333332e-2_f64) * t30247 - F::cast_from(0.66327777777777777775e-2_f64) * t26764 + F::cast_from(0.66327777777777777776e-2_f64) * t26785 + F::cast_from(0.33163888888888888887e-2_f64) * t26787 + F::cast_from(0.99491666666666666664e-2_f64) * t30254 + t13400 + F::cast_from(0.33163888888888888887e-2_f64) * t30258 - F::cast_from(0.66327777777777777776e-2_f64) * t30262 - F::cast_from(0.99491666666666666664e-2_f64) * t30264 + F::cast_from(0.66327777777777777776e-2_f64) * t30266 - F::cast_from(0.66327777777777777776e-2_f64) * t26841 - F::new(0.193e0) * t1220 * t30771 - F::new(0.223494e0) * t3930 * t30775 - F::cast_from(0.16581944444444444444e-2_f64) * t19788 + F::cast_from(0.24872916666666666666e-2_f64) * t30951;
    (t30947, t30951, t30953)
}
