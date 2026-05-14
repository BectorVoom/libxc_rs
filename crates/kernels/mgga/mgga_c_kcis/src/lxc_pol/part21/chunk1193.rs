//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1193/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1193<F: Float>(t28189: F, t3489: F, t28203: F, t2850: F, t5281: F, t5310: F, t15573: F, t28131: F, t7788: F, t96727: F, t27014: F, t28214: F, t95903: F, t26960: F, t28102: F, t7775: F, t7796: F, t8087: F, t92830: F, t93082: F, t95895: F, t95906: F) -> (F, F, F, F) {
    let t97010 = t28189 * t3489;
    let t97015 = t28203 * t3489;
    let t97019 = t5310 * t5281 * t2850;
    let t97024 = t15573 * t28131;
    let t97026 = 0.23168402777777777778e-3 * t7788 * t97024;
    let t97028 = 0.46336805555555555556e-3 * t7788 * t96727;
    let t97030 = 0.7722800925925925926e-4 * t27014 * t28214;
    let t97031 = 0.15476481481481481481e-2 * t95903;
    let t97033 = -0.24734586805555555556e-3 * t92830 * t8087 + 0.23214722222222222222e-2 * t95895 - 0.18534722222222222222e-2 * t97010 * t7796 - 0.18534722222222222222e-2 * t97010 * t7775 - 0.24734586805555555556e-3 * t97015 * t7775 - 0.23168402777777777778e-3 * t26960 * t97019 - 0.82448622685185185185e-4 * t93082 * t28102 - t97026 - t97028 - t97030 - t97031 + 0.61905925925925925925e-2 * t95906;
    (t97010, t97019, t97024, t97033)
}
