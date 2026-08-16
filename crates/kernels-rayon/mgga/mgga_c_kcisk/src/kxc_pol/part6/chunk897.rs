//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 897/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk897(t1801: f64, t28950: f64, t1873: f64, t1869: f64, t23969: f64, t23976: f64, t23978: f64, t28790: f64, t28794: f64, t28797: f64, t28803: f64, t28807: f64, t28811: f64, t28815: f64, t28818: f64) -> (f64, f64) {
    let t28951 = t1801 * t28950;
    let t28952 = t1873 * t28951;
    let t28953 = t1869 * t28952;
    let t28955 = 0.99491666666666666664e-2_f64 * t23969 + 0.99491666666666666664e-2_f64 * t28790 + 0.2653111111111111111e-1_f64 * t28794 + 0.2653111111111111111e-1_f64 * t28797 + 0.2653111111111111111e-1_f64 * t23976 - 0.16581944444444444444e-2_f64 * t28803 - 0.13265555555555555555e-1_f64 * t28807 - 0.22109259259259259258e-1_f64 * t28811 - 0.16581944444444444444e-1_f64 * t28815 - 0.99491666666666666664e-2_f64 * t28818 + 0.66327777777777777776e-2_f64 * t23978 - 0.24872916666666666666e-2_f64 * t28953;
    (t28953, t28955)
}
