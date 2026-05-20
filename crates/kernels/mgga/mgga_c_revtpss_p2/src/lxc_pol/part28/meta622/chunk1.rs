//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2202/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2202<F: Float>(t1646: F, t16561: F, t16591: F, t1695: F, t1976: F, t25460: F, t25473: F, t25586: F, t25591: F, t25631: F, t27427: F, t27594: F, t27598: F, t27639: F, t27643: F, t27665: F, t3046: F, t3060: F, t3075: F, t3270: F, t7144: F, t7145: F, t7147: F, t7156: F, t7159: F, t7160: F, t7817: F, t7818: F, t7828: F, t93436: F, t93498: F, t93502: F, t93904: F, t93968: F, t99675: F, t99684: F, t99685: F, t99709: F, t99721: F) -> F {
    let t99728 = -F::cast_from(0.17347256376410398924e1_f64) * t99675 * t25631 - F::cast_from(0.8673628188205199462e0_f64) * t7144 * t7145 * t25586 * t1646 + F::cast_from(0.26020884564615598386e1_f64) * t99684 * t99685 * t16561 + F::cast_from(0.8673628188205199462e0_f64) * t7156 * t27639 * t27643 + F::cast_from(0.34694512752820797848e1_f64) * t93436 * t27594 * t93498 + F::cast_from(0.8673628188205199462e0_f64) * t7159 * t7160 * t25586 * t1695 + F::cast_from(0.10408353825846239354e2_f64) * t7159 * t93968 * t7828 * t3270 + F::cast_from(0.34694512752820797848e1_f64) * t93502 * t27598 * t93498 + F::cast_from(0.17347256376410398924e1_f64) * t93904 * t27665 - F::cast_from(0.17347256376410398924e1_f64) * t99709 * t7147 + F::cast_from(0.17347256376410398924e1_f64) * t25473 * t27427 - F::cast_from(0.17347256376410398924e1_f64) * t3046 * t25460 * t7818 + F::cast_from(0.8673628188205199462e0_f64) * t7159 * t7160 * t1976 * t16591 + F::cast_from(0.13170898365871023197e1_f64) * t99721 * t3060 + F::cast_from(0.17347256376410398924e1_f64) * t25591 * t7145 * t7817 * t3075;
    t99728
}
