//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1114/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1114<F: Float>(t145074: F, t23701: F, t1013: F, t1554: F, t1008: F, t32186: F, t52: F, t1009: F, t1014: F, t105080: F, t105279: F, t138739: F, t138746: F, t139: F, t139057: F, t139065: F, t139132: F, t145171: F, t147253: F, t147412: F, t147416: F, t147425: F, t147429: F, t147533: F, t2036: F, t23810: F, t23831: F, t26700: F, t32241: F, t32791: F, t32797: F, t32815: F, t32822: F, t3347: F, t34884: F, t379: F, t40227: F, t5818: F, t8812: F, t8852: F) -> F {
    let t147542 = t23701 * t145074;
    let t147544 = t1554 * t1013;
    let t147572 = t52 * t32186 * t1008;
    let t147586 = F::new(0.10069900737806194568e-1) * t147542 - F::new(0.80027204934668021493e-1) * t138746 * t32241 * t147544 * t379 + F::new(0.53351469956445347664e-1) * t138739 * t32241 * t145171 * t26700 - F::new(0.45306850413028723348e0) * t32815 * t147533 - F::new(0.45306850413028723348e0) * t3347 * t139 * t34884 - F::new(0.14125722719362779755e-1) * t139057 + F::new(0.6041940442683716741e-1) * t139065 - F::new(0.27369475924647479993e1) * t8812 * t32791 * t1009 + F::new(0.21188584079044169633e-1) * t139132 * t105279 - F::new(0.21188584079044169633e-1) * t32797 * t105080 - F::new(0.20527106943485609994e0) * t8852 * t147425 - F::new(0.54738951849294959984e1) * t23810 * t147429 + F::new(0.24163653553615319119e1) * t32815 * t147572 + F::new(0.13684737962323739996e1) * t2036 * t32791 * t1014 - F::new(0.24163653553615319119e1) * t32822 * t147572 - F::new(0.91656519086197144464e-1) * t23831 * t147412 + F::new(0.45828259543098572232e-1) * t5818 * t147416 - F::new(0.41054213886971219988e0) * t40227 * t147253;
    t147586
}
