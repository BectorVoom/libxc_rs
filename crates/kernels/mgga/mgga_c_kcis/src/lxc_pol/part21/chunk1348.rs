//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1348/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1348<F: Float>(t7788: F, t96812: F, t95815: F, t11072: F, t1646: F, t26960: F, t26961: F, t28102: F, t3532: F, t8095: F, t92590: F, t92749: F, t92752: F, t92785: F, t93023: F, t95802: F, t95805: F, t95817: F, t95820: F) -> F {
    let t96875 = t7788 * t96812;
    let t96885 = F::new(0.15476481481481481481e-2) * t95815;
    let t96890 = F::new(0.69644166666666666664e-2) * t95802 - F::new(0.23214722222222222222e-2) * t95805 - F::new(0.20612155671296296296e-4) * t92749 - F::new(0.23168402777777777778e-3) * t92752 - F::new(0.7722800925925925926e-4) * t96875 + F::new(0.23168402777777777778e-3) * t93023 * t28102 - F::new(0.23168402777777777778e-3) * t26960 * t11072 * t26961 * t1646 * t3532 - F::new(0.23168402777777777778e-3) * t92785 - t96885 - F::new(0.51588271604938271604e-3) * t95817 - F::new(0.23214722222222222222e-2) * t95820 + F::new(0.34752604166666666667e-3) * t92590 * t8095;
    t96890
}
