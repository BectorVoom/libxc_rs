//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 690/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk690(t262: f64, t9888: f64, t7641: f64, t7648: f64, t9885: f64, t7653: f64, t3826: f64, t9708: f64, t3851: f64, t7583: f64, t8714: f64, t9445: f64, t9447: f64, t9448: f64, t9457: f64, t9874: f64, t9878: f64, t9880: f64, t9882: f64, t9886: f64) -> (f64, f64) {
    let t9889 = t262 * t9888;
    let t9890 = t7641 * t9889;
    let t9892 = t7648 * t9885;
    let t9894 = t7653 * t9889;
    let t9897 = t3826 * t9708;
    let t9899 = t3851 * t9708;
    let t9901 = 0.34093327067806677162e-2_f64 * t9874 - 0.45457769423742236216e-2_f64 * t9878 + 0.9072038638458063915e-4_f64 * t9880 - 0.10584045078201074568e-3_f64 * t9882 + 0.68186654135613354324e-2_f64 * t9886 + 0.22728884711871118108e-1_f64 * t9890 + 0.45360193192290319575e-3_f64 * t9892 + 0.84672360625608596544e-3_f64 * t9894 - t9445 + 0.56448240417072397695e-3_f64 * t8714 - t9447 + t9448 + 0.39828462315181744016e-2_f64 * t9897 + 0.5987120850931904282e-1_f64 * t9899 + t7583 + t9457;
    (t9889, t9901)
}
