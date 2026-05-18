//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 844/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk844<F: Float>(t38775: F, t194: F, t1979: F, t1982: F, t201: F, t5530: F, t2134: F, t27: F, t3118: F, t551: F, t1364: F, t2024: F, t31043: F, t34753: F, t34757: F, t38739: F, t38742: F, t38747: F, t38749: F, t38752: F, t38755: F, t38757: F, t38760: F, t38764: F, t5187: F, t5194: F, t665: F, t884: F, t903: F) -> F {
    let t38776 = F::new(0.18183107769496894486e-1) * t38775;
    let t38780 = t194 * t5530 * t201 * t1979 * t1982;
    let t38784 = t2134 * t27 * t3118 * t551;
    let t38786 = -F::new(0.8980681276397856423e-1) * t38739 - F::new(0.44903406381989282115e-1) * t38742 - t34753 - F::new(0.1616301098968908129e-5) * t34757 - F::new(0.81823984962736025184e-1) * t38747 + F::new(0.15243824895787514157e-3) * t38749 - F::new(0.36021158228745895953e-3) * t38752 - F::new(0.36021158228745895953e-3) * t38755 - F::new(0.15243824895787514157e-3) * t38757 - F::new(0.36021158228745895953e-3) * t38760 - F::new(0.36021158228745895953e-3) * t38764 + F::new(0.35922725105591425692e0) * t903 * t665 * t5187 - F::new(0.47896966807455234256e0) * t1364 * t665 * t5194 - F::new(0.23948483403727617128e0) * t884 * t2024 * t31043 + t38776 + F::new(0.42564599893297839398e-5) * t38780 + F::new(0.10000709273223291967e0) * t38784;
    t38786
}
