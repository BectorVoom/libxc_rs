//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 798/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk798(t138: f64, t32848: f64, t1691: f64, t32318: f64, t554: f64, t71: f64, t420: f64, t7195: f64, t140: f64, t2036: f64, t23701: f64, t23711: f64, t23721: f64, t23742: f64, t23745: f64, t23825: f64, t23832: f64, t23839: f64, t23842: f64, t32208: f64, t32215: f64, t32234: f64, t32803: f64, t32815: f64, t32817: f64, t32822: f64, t32836: f64, t32839: f64, t3392: f64, t539: f64, t543: f64, t555: f64, t5818: f64, t7318: f64, t8812: f64) -> (f64, f64, f64) {
    let t32849 = t32848 * t138;
    let t32852 = t138 * t1691;
    let t32853 = t32852 * t32318;
    let t32858 = t71 * t554;
    let t32859 = t420 * t32858;
    let t32860 = t7195 * t32859;
    let t32867 = 0.21188584079044169633e-1_f64 * t23745 * t32215 - 0.45306850413028723348e0_f64 * t32815 * t32817 - 0.42377168158088339266e-1_f64 * t23742 * t32215 + 0.45306850413028723348e0_f64 * t32822 * t32817 - 0.22653425206514361674e0_f64 * t23742 * t32803 - 0.10263553471742804997e0_f64 * t2036 * t7318 * t555 + 0.20527106943485609994e0_f64 * t8812 * t7318 * t539 - 0.27369475924647479993e1_f64 * t140 * t32208 + t32836 - 0.18125821328051150223e0_f64 * t23832 * t32839 + 0.54738951849294959985e1_f64 * t543 * t32208 - 0.30209702213418583705e-1_f64 * t23711 * t32234 + 0.30209702213418583705e-1_f64 * t23701 * t32234 + 0.22914129771549286116e-1_f64 * t23721 * t32849 - 0.45828259543098572232e-1_f64 * t5818 * t32853 + 0.15276086514366190744e-1_f64 * t3392 * t32853 + 0.18125821328051150223e0_f64 * t23842 * t32860 + 0.18125821328051150223e0_f64 * t23839 * t32839 - 0.18125821328051150223e0_f64 * t23825 * t32860;
    (t32852, t32858, t32867)
}
