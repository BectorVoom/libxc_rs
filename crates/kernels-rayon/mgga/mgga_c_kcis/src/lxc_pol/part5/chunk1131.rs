//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1131/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1131(t13710: f64, t13945: f64, t13949: f64, t18924: f64, t18927: f64, t18930: f64, t18933: f64, t18935: f64, t18937: f64, t9851: f64, t9852: f64, t13714: f64, t13717: f64, t13912: f64, t14001: f64, t14015: f64, t18650: f64, t18655: f64, t18659: f64, t18664: f64, t18667: f64, t18877: f64, t18880: f64, t18885: f64, t18887: f64, t18890: f64, t18909: f64, t18912: f64, t18920: f64, t19071: f64, t9691: f64, t9708: f64) -> f64 {
    let t19092 = -0.49671e0_f64 * t18924 + 0.66228e0_f64 * t18927 + 0.16557e0_f64 * t18930 - 0.27595e-1_f64 * t18933 - t9851 - t9852 - 0.11038e0_f64 * t18935 + 0.5519e-1_f64 * t18937 - 0.18396666666666666667e0_f64 * t13945 - 0.26837777777777777779e0_f64 * t13710 + 0.22076e0_f64 * t13949;
    let t19094 = -0.13418888888888888889e0_f64 * t9691 - 0.91983333333333333333e-1_f64 * t9708 + 0.18396666666666666667e-1_f64 * t18877 - 0.82785e-1_f64 * t18880 - t14001 + 0.36793333333333333333e-1_f64 * t13912 + 0.40256666666666666668e0_f64 * t13717 - 0.412621875e-1_f64 * t18885 + 0.16504875e0_f64 * t18887 + 0.82524375e-1_f64 * t18890 + t19071 - 0.36793333333333333333e-1_f64 * t18909 - 0.11038e0_f64 * t18912 + t14015 - 0.40256666666666666668e0_f64 * t13714 - 0.33547222222222222222e0_f64 * t18650 + 0.12077e1_f64 * t18655 - 0.80513333333333333332e0_f64 * t18659 - 0.181155e1_f64 * t18664 + 0.24154e1_f64 * t18667 + 0.16504875e0_f64 * t18920 + t19092;
    t19094
}
