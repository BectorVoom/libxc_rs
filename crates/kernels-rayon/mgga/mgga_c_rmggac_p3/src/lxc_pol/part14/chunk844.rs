//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 844/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk844(t38775: f64, t194: f64, t1979: f64, t1982: f64, t201: f64, t5530: f64, t2134: f64, t27: f64, t3118: f64, t551: f64, t1364: f64, t2024: f64, t31043: f64, t34753: f64, t34757: f64, t38739: f64, t38742: f64, t38747: f64, t38749: f64, t38752: f64, t38755: f64, t38757: f64, t38760: f64, t38764: f64, t5187: f64, t5194: f64, t665: f64, t884: f64, t903: f64) -> f64 {
    let t38776 = 0.18183107769496894486e-1_f64 * t38775;
    let t38780 = t194 * t5530 * t201 * t1979 * t1982;
    let t38784 = t2134 * t27 * t3118 * t551;
    let t38786 = -0.8980681276397856423e-1_f64 * t38739 - 0.44903406381989282115e-1_f64 * t38742 - t34753 - 0.1616301098968908129e-5_f64 * t34757 - 0.81823984962736025184e-1_f64 * t38747 + 0.15243824895787514157e-3_f64 * t38749 - 0.36021158228745895953e-3_f64 * t38752 - 0.36021158228745895953e-3_f64 * t38755 - 0.15243824895787514157e-3_f64 * t38757 - 0.36021158228745895953e-3_f64 * t38760 - 0.36021158228745895953e-3_f64 * t38764 + 0.35922725105591425692e0_f64 * t903 * t665 * t5187 - 0.47896966807455234256e0_f64 * t1364 * t665 * t5194 - 0.23948483403727617128e0_f64 * t884 * t2024 * t31043 + t38776 + 0.42564599893297839398e-5_f64 * t38780 + 0.10000709273223291967e0_f64 * t38784;
    t38786
}
