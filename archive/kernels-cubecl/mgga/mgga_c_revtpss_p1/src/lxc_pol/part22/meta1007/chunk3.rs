//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3446/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3446<F: Float>(t4746: F, t4930: F, t3075: F, t6244: F, t1000: F, t1076: F, t1079: F, t11128: F, t11201: F, t15648: F, t16242: F, t16255: F, t16284: F, t16292: F, t16295: F, t16312: F, t16313: F, t16344: F, t16591: F, t16600: F, t1695: F, t19385: F, t19403: F, t3063: F, t3269: F, t4935: F, t52927: F, t55458: F, t6251: F, t995: F, t996: F) -> (F, F) {
    let t64764 = t4746 * t4930;
    let t64772 = t6244 * t3075;
    let t64788 = F::cast_from(0.26341796731742046394e1_f64) * t11128 * t6251 + F::cast_from(0.13170898365871023197e1_f64) * t995 * t1079 * t15648 * t1695 + F::cast_from(0.26341796731742046394e1_f64) * t1076 * t3269 * t1695 * t16591 - F::cast_from(0.26341796731742046394e1_f64) * t64764 * t1000 - F::cast_from(0.52683593463484092788e1_f64) * t16312 * t16313 * t16242 + F::cast_from(0.52683593463484092788e1_f64) * t4935 * t16255 - F::cast_from(0.39512695097613069591e1_f64) * t11201 * t996 * t64772 + F::cast_from(0.52683593463484092788e1_f64) * t16600 * t16292 + F::cast_from(0.26341796731742046394e1_f64) * t16600 * t16295 - F::cast_from(0.52683593463484092788e1_f64) * t55458 * t19403 - F::cast_from(0.26341796731742046394e1_f64) * t16284 * t16344 + F::cast_from(0.13170898365871023197e1_f64) * t3063 * t19385 - F::cast_from(0.52683593463484092788e1_f64) * t52927 * t19403;
    (t64772, t64788)
}
