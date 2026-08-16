//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1110/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1110(t27028: f64, t6774: f64, t5329: f64, t6837: f64, t7794: f64, t1851: f64, t1856: f64, t26996: f64, t26955: f64, t28176: f64, t28190: f64, t28215: f64, t28925: f64, t28936: f64, t29123: f64, t29127: f64, t7772: f64, t7788: f64, t8091: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29147 = t27028 * t6774;
    let t29148 = t5329 * t29147;
    let t29151 = t7794 * t6837;
    let t29152 = t5329 * t29151;
    let t29159 = t1856 * t1851;
    let t29160 = t26996 * t29159;
    let t29161 = t5329 * t29160;
    let t29170 = 0.46377350260416666667e-4_f64 * t7772 * t29127 - 0.69505208333333333334e-3_f64 * t7788 * t29148 + 0.34752604166666666667e-3_f64 * t7788 * t29152 - 0.23214722222222222222e-2_f64 * t28925 + 0.30918233506944444444e-4_f64 * t26955 * t29123 - 0.23168402777777777778e-3_f64 * t28176 - 0.92754700520833333334e-4_f64 * t7772 * t29161 - 0.69505208333333333334e-3_f64 * t7788 * t29161 - 0.23168402777777777778e-3_f64 * t28190 * t8091 - 0.7722800925925925926e-4_f64 * t28215 + 0.15476481481481481481e-2_f64 * t28936;
    (t29147, t29148, t29151, t29152, t29159, t29160, t29161, t29170)
}
