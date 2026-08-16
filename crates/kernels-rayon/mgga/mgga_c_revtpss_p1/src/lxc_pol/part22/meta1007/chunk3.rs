//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3446/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3446(t4746: f64, t4930: f64, t3075: f64, t6244: f64, t1000: f64, t1076: f64, t1079: f64, t11128: f64, t11201: f64, t15648: f64, t16242: f64, t16255: f64, t16284: f64, t16292: f64, t16295: f64, t16312: f64, t16313: f64, t16344: f64, t16591: f64, t16600: f64, t1695: f64, t19385: f64, t19403: f64, t3063: f64, t3269: f64, t4935: f64, t52927: f64, t55458: f64, t6251: f64, t995: f64, t996: f64) -> (f64, f64) {
    let t64764 = t4746 * t4930;
    let t64772 = t6244 * t3075;
    let t64788 = 0.26341796731742046394e1_f64 * t11128 * t6251 + 0.13170898365871023197e1_f64 * t995 * t1079 * t15648 * t1695 + 0.26341796731742046394e1_f64 * t1076 * t3269 * t1695 * t16591 - 0.26341796731742046394e1_f64 * t64764 * t1000 - 0.52683593463484092788e1_f64 * t16312 * t16313 * t16242 + 0.52683593463484092788e1_f64 * t4935 * t16255 - 0.39512695097613069591e1_f64 * t11201 * t996 * t64772 + 0.52683593463484092788e1_f64 * t16600 * t16292 + 0.26341796731742046394e1_f64 * t16600 * t16295 - 0.52683593463484092788e1_f64 * t55458 * t19403 - 0.26341796731742046394e1_f64 * t16284 * t16344 + 0.13170898365871023197e1_f64 * t3063 * t19385 - 0.52683593463484092788e1_f64 * t52927 * t19403;
    (t64772, t64788)
}
