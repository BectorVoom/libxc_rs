//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 959/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk959<F: Float>(t41716: F, t41722: F, t41725: F, t4961: F, t702: F, t1668: F, t2265: F, t289: F, t36590: F, t36594: F, t37950: F, t41701: F, t41706: F, t41713: F, t41719: F, t41727: F, t41730: F, t41735: F, t41739: F, t530: F, t5355: F, t8048: F, t931: F, t9343: F) -> (F,) {
    let t43810 = 0.19158786722982093702e1 * t41716;
    let t43812 = 0.3193131120497015617e0 * t41722;
    let t43813 = 0.95793933614910468512e0 * t41725;
    let t43817 = t4961 * t702;
    let t43827 = -0.4726e1 * t530 * t37950 - 0.1276937996798935182e-3 * t41701 - 0.5107751987195740728e-4 * t41706 + 0.36366215538993788974e-1 * t36590 + 0.18183107769496894487e-1 * t36594 + 0.17961362552795712846e1 * t41713 + t43810 - 0.11974241701863808564e0 * t41719 - t43812 - t43813 + 0.66671395154821946452e-1 * t41727 - 0.2363e1 * t931 * t9343 - 0.4726e1 * t289 * t43817 - 0.85129199786595678799e-5 * t41730 - 0.72732431077987577947e-1 * t41735 - 0.40911992481368012595e-1 * t41739 - 0.2363e1 * t5355 * t2265 - 0.4726e1 * t1668 * t8048;
    (t43827,)
}
