//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 637/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk637<F: Float>(t262: F, t9888: F, t7641: F, t7648: F, t9885: F, t7653: F, t3826: F, t9708: F, t3851: F, t7583: F, t8714: F, t9445: F, t9447: F, t9448: F, t9457: F, t9874: F, t9878: F, t9880: F, t9882: F, t9886: F) -> (F, F) {
    let t9889 = t262 * t9888;
    let t9890 = t7641 * t9889;
    let t9892 = t7648 * t9885;
    let t9894 = t7653 * t9889;
    let t9897 = t3826 * t9708;
    let t9899 = t3851 * t9708;
    let t9901 = 0.34093327067806677162e-2 * t9874 - 0.45457769423742236216e-2 * t9878 + 0.9072038638458063915e-4 * t9880 - 0.10584045078201074568e-3 * t9882 + 0.68186654135613354324e-2 * t9886 + 0.22728884711871118108e-1 * t9890 + 0.45360193192290319575e-3 * t9892 + 0.84672360625608596544e-3 * t9894 - t9445 + 0.56448240417072397695e-3 * t8714 - t9447 + t9448 + 0.39828462315181744016e-2 * t9897 + 0.5987120850931904282e-1 * t9899 + t7583 + t9457;
    (t9889, t9901)
}
