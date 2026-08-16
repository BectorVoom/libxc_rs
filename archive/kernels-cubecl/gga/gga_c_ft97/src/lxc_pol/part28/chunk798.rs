//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 798/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk798<F: Float>(t138: F, t32848: F, t1691: F, t32318: F, t554: F, t71: F, t420: F, t7195: F, t140: F, t2036: F, t23701: F, t23711: F, t23721: F, t23742: F, t23745: F, t23825: F, t23832: F, t23839: F, t23842: F, t32208: F, t32215: F, t32234: F, t32803: F, t32815: F, t32817: F, t32822: F, t32836: F, t32839: F, t3392: F, t539: F, t543: F, t555: F, t5818: F, t7318: F, t8812: F) -> (F, F, F) {
    let t32849 = t32848 * t138;
    let t32852 = t138 * t1691;
    let t32853 = t32852 * t32318;
    let t32858 = t71 * t554;
    let t32859 = t420 * t32858;
    let t32860 = t7195 * t32859;
    let t32867 = F::cast_from(0.21188584079044169633e-1_f64) * t23745 * t32215 - F::cast_from(0.45306850413028723348e0_f64) * t32815 * t32817 - F::cast_from(0.42377168158088339266e-1_f64) * t23742 * t32215 + F::cast_from(0.45306850413028723348e0_f64) * t32822 * t32817 - F::cast_from(0.22653425206514361674e0_f64) * t23742 * t32803 - F::cast_from(0.10263553471742804997e0_f64) * t2036 * t7318 * t555 + F::cast_from(0.20527106943485609994e0_f64) * t8812 * t7318 * t539 - F::cast_from(0.27369475924647479993e1_f64) * t140 * t32208 + t32836 - F::cast_from(0.18125821328051150223e0_f64) * t23832 * t32839 + F::cast_from(0.54738951849294959985e1_f64) * t543 * t32208 - F::cast_from(0.30209702213418583705e-1_f64) * t23711 * t32234 + F::cast_from(0.30209702213418583705e-1_f64) * t23701 * t32234 + F::cast_from(0.22914129771549286116e-1_f64) * t23721 * t32849 - F::cast_from(0.45828259543098572232e-1_f64) * t5818 * t32853 + F::cast_from(0.15276086514366190744e-1_f64) * t3392 * t32853 + F::cast_from(0.18125821328051150223e0_f64) * t23842 * t32860 + F::cast_from(0.18125821328051150223e0_f64) * t23839 * t32839 - F::cast_from(0.18125821328051150223e0_f64) * t23825 * t32860;
    (t32852, t32858, t32867)
}
