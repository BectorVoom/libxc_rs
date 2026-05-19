//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1131/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1131<F: Float>(t13710: F, t13945: F, t13949: F, t18924: F, t18927: F, t18930: F, t18933: F, t18935: F, t18937: F, t9851: F, t9852: F, t13714: F, t13717: F, t13912: F, t14001: F, t14015: F, t18650: F, t18655: F, t18659: F, t18664: F, t18667: F, t18877: F, t18880: F, t18885: F, t18887: F, t18890: F, t18909: F, t18912: F, t18920: F, t19071: F, t9691: F, t9708: F) -> F {
    let t19092 = -F::new(0.49671e0) * t18924 + F::new(0.66228e0) * t18927 + F::new(0.16557e0) * t18930 - F::new(0.27595e-1) * t18933 - t9851 - t9852 - F::new(0.11038e0) * t18935 + F::new(0.5519e-1) * t18937 - F::cast_from(0.18396666666666666667e0_f64) * t13945 - F::cast_from(0.26837777777777777779e0_f64) * t13710 + F::new(0.22076e0) * t13949;
    let t19094 = -F::cast_from(0.13418888888888888889e0_f64) * t9691 - F::cast_from(0.91983333333333333333e-1_f64) * t9708 + F::cast_from(0.18396666666666666667e-1_f64) * t18877 - F::new(0.82785e-1) * t18880 - t14001 + F::cast_from(0.36793333333333333333e-1_f64) * t13912 + F::cast_from(0.40256666666666666668e0_f64) * t13717 - F::cast_from(0.412621875e-1_f64) * t18885 + F::new(0.16504875e0) * t18887 + F::new(0.82524375e-1) * t18890 + t19071 - F::cast_from(0.36793333333333333333e-1_f64) * t18909 - F::new(0.11038e0) * t18912 + t14015 - F::cast_from(0.40256666666666666668e0_f64) * t13714 - F::cast_from(0.33547222222222222222e0_f64) * t18650 + F::new(0.12077e1) * t18655 - F::cast_from(0.80513333333333333332e0_f64) * t18659 - F::new(0.181155e1) * t18664 + F::new(0.24154e1) * t18667 + F::new(0.16504875e0) * t18920 + t19092;
    t19094
}
