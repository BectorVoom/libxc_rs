//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1151/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1151<F: Float>(t34045: F, t9649: F, t32885: F, t32887: F, t33031: F, t33056: F, t34013: F, t34018: F, t34023: F, t34027: F, t34032: F, t34039: F, t2789: F, t6944: F, t415: F, t2528: F, t32965: F) -> (F, F, F, F) {
    let t34046 = t9649 * t34045;
    let t34049 = 0.13402777777777777778e-2 * t33056 * t34013 - 0.46296296296296296297e-2 * t33031 * t34018 + 0.34722222222222222223e-2 * t33031 * t34023 + 0.34722222222222222223e-2 * t33031 * t34027 + 0.34722222222222222223e-2 * t33031 * t34032 + 0.34722222222222222223e-2 * t33031 * t34013 + 0.69444444444444444446e-2 * t33031 * t34039 + 0.13402777777777777778e-2 * t33056 * t34027 - 0.34722222222222222223e-2 * t32885 + 0.13402777777777777778e-2 * t34046 - 0.34722222222222222223e-2 * t32887;
    let t34054 = t6944 * t2789;
    let t34055 = t415 * t34054;
    let t34057 = t32965 * t2528;
    (t34049, t34054, t34055, t34057)
}
