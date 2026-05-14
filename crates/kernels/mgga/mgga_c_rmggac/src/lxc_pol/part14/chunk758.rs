//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 758/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk758<F: Float>(t1364: F, t2024: F, t31043: F, t34753: F, t34757: F, t38739: F, t38742: F, t38747: F, t38749: F, t38752: F, t38755: F, t38757: F, t38760: F, t38764: F, t38776: F, t38780: F, t38784: F, t5187: F, t5194: F, t665: F, t884: F, t903: F) -> (F,) {
    let t38786 = -0.8980681276397856423e-1 * t38739 - 0.44903406381989282115e-1 * t38742 - t34753 - 0.1616301098968908129e-5 * t34757 - 0.81823984962736025184e-1 * t38747 + 0.15243824895787514157e-3 * t38749 - 0.36021158228745895953e-3 * t38752 - 0.36021158228745895953e-3 * t38755 - 0.15243824895787514157e-3 * t38757 - 0.36021158228745895953e-3 * t38760 - 0.36021158228745895953e-3 * t38764 + 0.35922725105591425692e0 * t903 * t665 * t5187 - 0.47896966807455234256e0 * t1364 * t665 * t5194 - 0.23948483403727617128e0 * t884 * t2024 * t31043 + t38776 + 0.42564599893297839398e-5 * t38780 + 0.10000709273223291967e0 * t38784;
    (t38786,)
}
