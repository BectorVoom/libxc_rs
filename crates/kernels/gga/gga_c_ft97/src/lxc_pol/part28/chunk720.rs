//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 720/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk720<F: Float>(t32858: F, t420: F, t7195: F, t140: F, t2036: F, t23701: F, t23711: F, t23721: F, t23742: F, t23745: F, t23825: F, t23832: F, t23839: F, t23842: F, t32208: F, t32215: F, t32234: F, t32803: F, t32815: F, t32817: F, t32822: F, t32836: F, t32839: F, t32849: F, t32853: F, t3392: F, t539: F, t543: F, t555: F, t5818: F, t7318: F, t8812: F) -> (F,) {
    let t32859 = t420 * t32858;
    let t32860 = t7195 * t32859;
    let t32867 = 0.21188584079044169633e-1 * t23745 * t32215 - 0.45306850413028723348e0 * t32815 * t32817 - 0.42377168158088339266e-1 * t23742 * t32215 + 0.45306850413028723348e0 * t32822 * t32817 - 0.22653425206514361674e0 * t23742 * t32803 - 0.10263553471742804997e0 * t2036 * t7318 * t555 + 0.20527106943485609994e0 * t8812 * t7318 * t539 - 0.27369475924647479993e1 * t140 * t32208 + t32836 - 0.18125821328051150223e0 * t23832 * t32839 + 0.54738951849294959985e1 * t543 * t32208 - 0.30209702213418583705e-1 * t23711 * t32234 + 0.30209702213418583705e-1 * t23701 * t32234 + 0.22914129771549286116e-1 * t23721 * t32849 - 0.45828259543098572232e-1 * t5818 * t32853 + 0.15276086514366190744e-1 * t3392 * t32853 + 0.18125821328051150223e0 * t23842 * t32860 + 0.18125821328051150223e0 * t23839 * t32839 - 0.18125821328051150223e0 * t23825 * t32860;
    (t32867,)
}
